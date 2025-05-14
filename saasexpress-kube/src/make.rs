use futures_util::StreamExt;
use reqwest::{Certificate, Client, Identity, header};
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::{error::Error, fs, path::PathBuf};

// Define the Pod structures we'll need
#[derive(Serialize, Debug)]
struct PodCreate {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: Metadata,
    spec: PodSpec,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    namespace: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
struct PodSpec {
    containers: Vec<Container>,
    volumes: Option<Vec<Volume>>,
}

#[derive(Serialize, Debug)]
struct Volume {
    name: String,
    empty_dir: Option<EmptyDir>,
}

#[derive(Serialize, Debug)]
struct EmptyDir {
    medium: Option<String>,
    size_limit: Option<String>,
}

#[derive(Serialize, Debug)]
struct Container {
    name: String,
    image: String,
    ports: Vec<Port>,

    #[serde(rename = "volumeMounts")]
    volume_mounts: Option<Vec<VolumeMount>>,
}

#[derive(Serialize, Debug)]
struct VolumeMount {
    name: String,
    #[serde(rename = "mountPath")]
    mount_path: String,
    #[serde(rename = "subPath")]
    sub_path: Option<String>,
}

#[derive(Serialize, Debug)]
struct Port {
    #[serde(rename = "containerPort")]
    container_port: u16,
    name: String,
}

// For parsing watch events
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WatchEvent {
    #[serde(rename = "type")]
    event_type: String,
    object: PodObject,
}

#[derive(Deserialize, Debug)]
struct PodObject {
    status: PodStatus,
    metadata: PodMetadata,
}

#[derive(Deserialize, Debug)]
struct PodMetadata {
    name: String,
}

#[derive(Deserialize, Debug)]
struct PodStatus {
    phase: String,
}

// Kubeconfig structures
#[derive(Deserialize, Debug)]
struct KubeConfig {
    clusters: Vec<ClusterEntry>,
    contexts: Vec<ContextEntry>,
    #[serde(rename = "current-context")]
    current_context: String,
    users: Vec<UserEntry>,
}

#[derive(Deserialize, Debug)]
struct ClusterEntry {
    name: String,
    cluster: ClusterConfig,
}

#[derive(Deserialize, Debug)]
struct ClusterConfig {
    server: String,
    #[serde(rename = "certificate-authority-data", default)]
    ca_data: Option<String>,
    #[serde(rename = "certificate-authority", default)]
    ca_file: Option<String>,
    #[serde(rename = "insecure-skip-tls-verify", default)]
    insecure_skip_tls_verify: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct ContextEntry {
    name: String,
    context: ContextConfig,
}

#[derive(Deserialize, Debug)]
struct ContextConfig {
    cluster: String,
    user: String,
    namespace: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UserEntry {
    name: String,
    user: UserConfig,
}

#[derive(Deserialize, Debug)]
struct UserConfig {
    #[serde(rename = "client-certificate-data", default)]
    client_cert_data: Option<String>,
    #[serde(rename = "client-key-data", default)]
    client_key_data: Option<String>,
    #[serde(rename = "client-certificate", default)]
    client_cert_file: Option<String>,
    #[serde(rename = "client-key", default)]
    client_key_file: Option<String>,
    #[serde(rename = "token", default)]
    token: Option<String>,
    #[serde(rename = "tokenFile", default)]
    token_file: Option<String>,
    #[serde(rename = "username", default)]
    username: Option<String>,
    #[serde(rename = "password", default)]
    password: Option<String>,
}

struct KubeClientConfig {
    server: String,
    namespace: String,
    token: Option<String>,
    certificate_authority: Option<Vec<u8>>,
    client_certificate: Option<Vec<u8>>,
    client_key: Option<Vec<u8>>,
    username: Option<String>,
    password: Option<String>,
    insecure_skip_tls_verify: bool,
}

// Attempt to load kubeconfig from various sources
fn load_kube_config() -> Result<KubeClientConfig, Box<dyn Error>> {
    // Try environment variable first
    let kube_config_path = env::var("KUBECONFIG").unwrap_or_else(|_| {
        let home = dirs::home_dir().expect("Failed to determine home directory");
        home.join(".kube/config").to_string_lossy().to_string()
    });

    // Read and parse kubeconfig
    let config_data = fs::read_to_string(&kube_config_path)?;
    let kube_config: KubeConfig = serde_yaml::from_str(&config_data)?;

    // Find current context
    let context = kube_config
        .contexts
        .iter()
        .find(|c| c.name == kube_config.current_context)
        .ok_or("Current context not found in kubeconfig")?;

    // Get cluster config for context
    let cluster = kube_config
        .clusters
        .iter()
        .find(|c| c.name == context.context.cluster)
        .ok_or("Cluster not found for context")?;

    // Get user config for context
    let user = kube_config
        .users
        .iter()
        .find(|u| u.name == context.context.user)
        .ok_or("User not found for context")?;

    // Determine namespace (default to "default" if not specified)
    let namespace = context
        .context
        .namespace
        .clone()
        .unwrap_or_else(|| "default".to_string());

    // Process certificate authority data
    let certificate_authority = match (&cluster.cluster.ca_data, &cluster.cluster.ca_file) {
        (Some(data), _) => Some(base64::decode(data)?),
        (_, Some(path)) => {
            let path = if path.starts_with('/') {
                PathBuf::from(path)
            } else {
                PathBuf::from(&kube_config_path)
                    .parent()
                    .unwrap_or(&PathBuf::from("."))
                    .join(path)
            };
            Some(fs::read(path)?)
        }
        _ => None,
    };

    // Process client certificate and key data
    let (client_certificate, client_key) = match (
        &user.user.client_cert_data,
        &user.user.client_key_data,
        &user.user.client_cert_file,
        &user.user.client_key_file,
    ) {
        (Some(cert_data), Some(key_data), _, _) => (
            Some(base64::decode(cert_data)?),
            Some(base64::decode(key_data)?),
        ),
        (_, _, Some(cert_path), Some(key_path)) => {
            let cert_path = if cert_path.starts_with('/') {
                PathBuf::from(cert_path)
            } else {
                PathBuf::from(&kube_config_path)
                    .parent()
                    .unwrap_or(&PathBuf::from("."))
                    .join(cert_path)
            };

            let key_path = if key_path.starts_with('/') {
                PathBuf::from(key_path)
            } else {
                PathBuf::from(&kube_config_path)
                    .parent()
                    .unwrap_or(&PathBuf::from("."))
                    .join(key_path)
            };

            (Some(fs::read(cert_path)?), Some(fs::read(key_path)?))
        }
        _ => (None, None),
    };

    // Get token or token file
    let token = match (&user.user.token, &user.user.token_file) {
        (Some(token), _) => Some(token.clone()),
        (_, Some(token_file)) => {
            let token_file_path = if token_file.starts_with('/') {
                PathBuf::from(token_file)
            } else {
                PathBuf::from(&kube_config_path)
                    .parent()
                    .unwrap_or(&PathBuf::from("."))
                    .join(token_file)
            };
            Some(fs::read_to_string(token_file_path)?.trim().to_string())
        }
        _ => None,
    };

    // Return the client config
    Ok(KubeClientConfig {
        server: cluster.cluster.server.clone(),
        namespace,
        token,
        certificate_authority,
        client_certificate,
        client_key,
        username: user.user.username.clone(),
        password: user.user.password.clone(),
        insecure_skip_tls_verify: cluster.cluster.insecure_skip_tls_verify.unwrap_or(false),
    })
}

// Try to get in-cluster config (when running in a pod)
fn load_in_cluster_config() -> Result<KubeClientConfig, Box<dyn Error>> {
    const SERVICE_HOST_ENV: &str = "KUBERNETES_SERVICE_HOST";
    const SERVICE_PORT_ENV: &str = "KUBERNETES_SERVICE_PORT";
    const TOKEN_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";
    const CA_CERT_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt";
    const NAMESPACE_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/namespace";

    // Check if running in a pod
    let host = env::var(SERVICE_HOST_ENV)?;
    let port = env::var(SERVICE_PORT_ENV)?;

    // Read token, cert, and namespace from files
    let token = fs::read_to_string(TOKEN_FILE)?;
    let certificate_authority = Some(fs::read(CA_CERT_FILE)?);
    let namespace = fs::read_to_string(NAMESPACE_FILE)?;

    let server = format!("https://{}:{}", host, port);

    Ok(KubeClientConfig {
        server,
        namespace: namespace.trim().to_string(),
        token: Some(token.trim().to_string()),
        certificate_authority,
        client_certificate: None,
        client_key: None,
        username: None,
        password: None,
        insecure_skip_tls_verify: false,
    })
}

// Build HTTP client from kube config
async fn build_client(config: &KubeClientConfig) -> Result<Client, Box<dyn Error>> {
    let mut client_builder =
        Client::builder().danger_accept_invalid_certs(config.insecure_skip_tls_verify);

    // Add CA certificate if available
    if let Some(ca_data) = &config.certificate_authority {
        let cert = Certificate::from_pem(ca_data)?;
        client_builder = client_builder.add_root_certificate(cert);
    }

    // Add client certificate and key if available
    if let (Some(cert_data), Some(key_data)) = (&config.client_certificate, &config.client_key) {
        // Make sure the cert and key are in proper PEM format
        // from_pem expects PEM-encoded data with proper headers
        let identity = Identity::from_pkcs8_pem(cert_data, key_data)?;
        client_builder = client_builder.identity(identity);
    }

    // Create default headers
    let mut headers = header::HeaderMap::new();
    // headers.insert(
    //     header::CONTENT_TYPE,
    //     header::HeaderValue::from_static("application/json"),
    // );

    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("rust-client"),
    );

    // Add auth header if token is available
    if let Some(token) = &config.token {
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
    } else if let (Some(username), Some(password)) = (&config.username, &config.password) {
        // Add basic auth if username/password available
        let auth = base64::encode(format!("{}:{}", username, password));
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", auth))?,
        );
    }

    let src = "application/json;as=Table;v=v1;g=meta.k8s.io,application/json;as=Table;v=v1beta1;g=meta.k8s.io,application/json";

    headers.insert(header::ACCEPT, header::HeaderValue::from_static(src));

    // headers.insert(
    //     header::ACCEPT,
    //     header::HeaderValue::from_static("application/json, application/vnd.kubernetes.protobuf, application/vnd.kubernetes.protobuf;stream=watch, application/json;stream=watch"),
    // );

    println!("Headers: {:?}", headers);
    let client = client_builder.default_headers(headers).build()?;

    Ok(client)
}

pub async fn main() -> Result<(), Box<dyn Error>> {
    // First try in-cluster config, fall back to kubeconfig
    let config = load_in_cluster_config().or_else(|_| load_kube_config())?;
    println!("Using Kubernetes API server: {}", config.server);
    println!("Using namespace: {}", config.namespace);

    // Build HTTP client
    let client = build_client(&config).await?;

    // Define the pod we want to create
    let pod = PodCreate {
        api_version: "v1".to_string(),
        kind: "Pod".to_string(),
        metadata: Metadata {
            name: "micpod".to_string(),
            namespace: config.namespace.clone(),
            labels: Some(
                [
                    (
                        "app.kubernetes.io/instance".to_string(),
                        "saase".to_string(),
                    ),
                    (
                        "app.kubernetes.io/name".to_string(),
                        "generic-api".to_string(),
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
        },
        spec: PodSpec {
            containers: vec![Container {
                name: "micpod".to_string(),
                image: "quay.io/saasexpress/saasexpress:feature-config-otel".to_string(),
                ports: vec![Port {
                    container_port: 2243,
                    name: "http".to_string(),
                }],
                volume_mounts: Some(vec![VolumeMount {
                    name: "temp".to_string(),
                    mount_path: "/var/cache/nginx".to_string(),
                    sub_path: None,
                }]),
            }],
            volumes: Some(vec![Volume {
                name: "temp".to_string(),
                empty_dir: Some(EmptyDir {
                    medium: None,
                    size_limit: None,
                }),
            }]),
        },
    };
    println!("{}", serde_yaml::to_string(&pod).unwrap());

    // Create the pod
    println!("Creating pod...");
    let create_url = format!(
        "{}/api/v1/namespaces/{}/pods",
        config.server, config.namespace
    );

    let response = client.post(&create_url).json(&pod).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("Failed to create pod: {} {}", status, error_text).into());
    }

    println!("Pod created successfully, watching for it to start...");

    Ok(())
}
