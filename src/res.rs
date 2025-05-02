// Copyright The OpenTelemetry Authors
// SPDX-License-Identifier: Apache-2.0

// use opentelemetry_sdk::{
//     Resource,
//     resource::{
//         EnvResourceDetector, OsResourceDetector, ProcessResourceDetector, ResourceDetector,
//         SdkProvidedResourceDetector, TelemetryResourceDetector,
//     },
// };
use std::time::Duration;

use opentelemetry_sdk::{Resource, resource::ResourceDetector};

// pub fn get_resource_attr() -> Resource {
//     let os_resource = ResourceDetector.detect(Duration::from_secs(0));
//     let process_resource = ProcessResourceDetector.detect(Duration::from_secs(0));
//     let sdk_resource = SdkProvidedResourceDetector.detect(Duration::from_secs(0));
//     let env_resource = EnvResourceDetector::new().detect(Duration::from_secs(0));
//     let telemetry_resource = TelemetryResourceDetector.detect(Duration::from_secs(0));

//     os_resource
//         .merge(&process_resource)
//         .merge(&sdk_resource)
//         .merge(&env_resource)
//         .merge(&telemetry_resource)
// }
