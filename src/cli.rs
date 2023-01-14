use std::{fmt, str};

use clap::{Parser, Subcommand, ValueEnum};
use seq_macro::seq;
use serde::{Deserialize, Serialize};

use crate::output;

seq!(N in 20..=24 {
    /// Kubernetes version(s) supported
    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum KubernetesVersion {
        #( V~N, )*
    }

    /// Formats the Kubernetes version as a string in the form of "1.X"
    impl fmt::Display for KubernetesVersion {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                #( KubernetesVersion::V~N => write!(f, "1.{}", N), )*
            }
        }
    }

    /// Used by clap for acceptable values and converting from input to enum
    impl ValueEnum for KubernetesVersion {
        fn value_variants<'a>() -> &'a [Self] {
            &[
                #( Self::V~N, )*
            ]
        }

        fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
            match self {
                #( Self::V~N => Some(clap::builder::PossibleValue::new(format!("1.{}", N))), )*
            }
        }
    }
});

/// Compute constructs supported by Amazon EKS the data plane
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Compute {
  EksManaged,
  SelfManaged,
  FargateProfile,
}

/// Used by clap for acceptable values and converting from input to enum
impl ValueEnum for Compute {
  fn value_variants<'a>() -> &'a [Self] {
    &[Self::EksManaged, Self::SelfManaged, Self::FargateProfile]
  }

  fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
    match self {
      Self::EksManaged => Some(clap::builder::PossibleValue::new("eks")),
      Self::SelfManaged => Some(clap::builder::PossibleValue::new("self")),
      Self::FargateProfile => Some(clap::builder::PossibleValue::new("fargate")),
    }
  }
}

/// The different types of strategies for upgrading a cluster
///
/// `InPlace`: the control plane is updated in-place by Amazon EKS
/// `BlueGreen`: an entirely new cluster is created alongside the existing
/// and the workloads+traffic will need to be migrated to the new cluster
#[derive(Clone, Copy, Debug, ValueEnum, Serialize, Deserialize)]
pub enum Strategy {
  InPlace,
  // BlueGreen,
}

/// The default cluster upgrade strategy is `InPlace`
impl Default for Strategy {
  fn default() -> Self {
    Self::InPlace
  }
}

/// Analyze an Amazon EKS cluster prior to upgrading
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Analysis {
  /// The name of the cluster to analyze
  #[arg(long, alias = "name", value_enum)]
  pub cluster_name: String,

  /// The AWS region where the cluster is provisioned
  #[arg(long)]
  pub region: Option<String>,

  #[arg(long, alias = "ofmt", value_enum, default_value_t)]
  pub output_format: output::OutputFormat,

  #[arg(long, alias = "otype", value_enum, default_value_t)]
  pub output_type: output::OutputType,

  #[arg(long, alias = "ofile")]
  pub output_filename: Option<String>,
}

/// Create a playbook for upgrading an Amazon EKS cluster
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Playbook {
  /// The name of the cluster
  #[arg(long, default_value = "<CLUSTER_NAME>")]
  pub cluster_name: Option<String>,

  /// The cluster's current Kubernetes version
  #[arg(long, value_enum)]
  pub cluster_version: KubernetesVersion,

  /// Array of compute types used in the data plane
  #[arg(long, value_enum, num_args = 1..=3)]
  pub compute: Vec<Compute>,

  /// Whether the AMI used is custom or not (provided by AWS)
  #[arg(long)]
  pub custom_ami: bool,

  /// Name of the output file
  #[arg(short, long, default_value = "playbook.md")]
  pub filename: String,

  /// The cluster upgrade strategy
  #[arg(short, long, value_enum, default_value_t)]
  pub strategy: Strategy,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  Analyze(Analysis),
  CreatePlaybook(Playbook),
}

#[derive(Parser, Debug)]
#[command(author, about, version)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}
