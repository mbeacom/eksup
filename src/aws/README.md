# AWS

## Control Plane

- Version
  - [x] [CHECK] Version skew between control plane and data plane
- Subnets
  - [x] [CHECK] Number of free IPs > 5

## Data Plane

- Subnets
  - [ ] [CHECK] Percentage of total IPs available
- EKS managed node group(s)
  - [ ] [CHECK] Pending updates (launch template version to be deployed)
  - [ ] [CHECK] AMI is custom or EKS optimized
- Self-managed node group(s)
  - [ ] [CHECK] Pending updates (launch template version to be deployed)
  - [ ] [CHECK] AMI is custom or EKS optimized
- Fargate Profile(s)

## Addons

- Version(s)
  - [ ] [CHECK] Pending updates
  - [ ] [CHECK] Default version for target Kubernetes version

## Misc

- Service limits
  - [ ] [CHECK] EC2 instance service limits
  - [ ] [CHECK] EBS volume service limits