### Registry

- A container registry is a **_service_** 👍 that stores and distributes container images and **_related artifacts_**. Docker Hub is an example of a public container **_registry_** that serves as a general catalog of Docker container images. 
- Pyrisa Container Registry provides users with automatic docker image registration, transparency log, integrated verification, decentralized distribution, and many other enhanced features.
- Question❓: Following Open Container Initiative(OCI) formats?

### Repository
- A repository is a collection of container images or other artifacts in a registry that have the same name, but different tags. For example, the following three images are in the acr-helloworld repository:
  - acr-helloworld:latest
  - acr-helloworld:v1
  - acr-helloworld:v2
- Repository names can also include namespaces. Namespaces allow you to identify related repositories and artifact ownership in your organization by using forward slash-delimited names. However, the registry manages all repositories independently, not as a hierarchy. For example:
  - marketing/campaign10-18/web:v2
  - marketing/campaign10-18/api:v3
  - product-returns/web-submission:20180604
  
  Repository names can only include lowercase alphanumeric characters, periods, dashes, underscores, and forward slashes.

  For complete repository naming rules, see the [Open Container Initiative Distribution Specification](https://github.com/distribution/distribution/blob/main/docs/spec/api.md#overview).

