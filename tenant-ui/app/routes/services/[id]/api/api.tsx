// src/App.js
import React, { useEffect, useState } from "react";
import yaml from "js-yaml"; // Install with `npm install js-yaml`
//import axios from "axios"; // Install with `npm install axios`

interface ApiProps {
  types: string[];
}

const Api = ({ types = [] }: ApiProps) => {
  const [openApiSpec, setOpenApiSpec] = useState<any>(null);

  // Fetch the OpenAPI YAML file
  useEffect(() => {
    const fetchOpenApiSpec = async () => {
      try {
        const response = await fetch("/api/openapi.yaml");
        if (!response.ok) {
          throw Error(await response.text());
        }
        const spec = yaml.load(await response.text()); // Parse YAML into JavaScript object
        setOpenApiSpec(spec);
      } catch (error) {
        console.error("Error fetching OpenAPI spec:", error);
      }
    };
    fetchOpenApiSpec();
  }, []);

  // Render the OpenAPI spec
  if (!openApiSpec) {
    return (
      <div className="text-center text-gray-500">
        Loading API Documentation...
      </div>
    );
  }

  if (!openApiSpec.info) {
    return (
      <div
        className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative"
        role="alert"
      >
        <strong className="font-bold">Invalid OpenAPI Spec</strong>
        <span className="block">
          Unable to get the API Specification from the service.
        </span>
      </div>
    );
  }
  return (
    <div className="p-4 container mx-auto">
      {/* Header */}
      {types.includes("header") && (
        <header className="mb-6">
          <h1 className="text-2xl font-bold">{openApiSpec.info.title}</h1>
          <p className="text-gray-600">{openApiSpec.info.description}</p>
        </header>
      )}

      {/* Paths */}
      {types.includes("endpoints") && (
        <section className="mb-6">
          <h2 className="text-xl font-semibold mb-4">Endpoints</h2>
          <div className="space-y-4">
            {Object.entries(openApiSpec.paths).map(([path, methods]: any) => (
              <div key={path} className="border rounded-lg p-4 bg-gray-50">
                <h3 className="font-mono text-lg text-blue-600">{path}</h3>
                <div className="space-y-2 mt-2">
                  {Object.entries(methods).map(([method, details]: any) => (
                    <div key={method}>
                      <span
                        className={`inline-block px-2 py-1 text-sm rounded ${
                          method === "get"
                            ? "bg-green-100 text-green-700"
                            : method === "post"
                            ? "bg-blue-100 text-blue-700"
                            : "bg-gray-100 text-gray-700"
                        }`}
                      >
                        {method.toUpperCase()}
                      </span>
                      <p className="mt-1 text-gray-700">
                        {details.summary || "No description provided."}
                      </p>
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Components/Schemas */}
      {types.includes("schemas") && (
        <section>
          <h2 className="text-xl font-semibold mb-4">Schemas</h2>
          <div className="space-y-4">
            {openApiSpec.components?.schemas &&
              Object.entries(openApiSpec.components.schemas).map(
                ([schemaName, schema]) => (
                  <div
                    key={schemaName}
                    className="border rounded-lg p-4 bg-gray-50"
                  >
                    <h3 className="font-mono text-lg text-blue-600">
                      {schemaName}
                    </h3>
                    <pre className="text-sm bg-gray-100 p-2 rounded">
                      {JSON.stringify(schema, null, 2)}
                    </pre>
                  </div>
                )
              )}
          </div>
        </section>
      )}
    </div>
  );
};

export default Api;
