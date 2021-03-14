use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;

use kube::{
    api::{Api, DeleteParams, ListParams, Meta, Patch, PatchParams, PostParams, WatchEvent},
    Client,
};

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn pod_works() -> anyhow::Result<()> {
        // simple_logger::SimpleLogger::new().init()?;
        std::env::set_var("RUST_LOG", "info,kube=debug");
        env_logger::init();

        // Create Pod blog
        info!("Creating Pod instance blog");
        let p: Pod = serde_json::from_value(json!({
            "apiVersion": "v1",
            "kind": "Pod",
            "metadata": { "name": "blog" },
            "spec": {
                "containers": [{
                  "name": "blog",
                  "image": "clux/blog:0.1.0"
                }],
            }
        }))?;

        assert_eq!(Some("blog".to_string()), p.metadata.name);
        println!("{:?}", p.metadata);

        let p1cpy = p.clone();
        if let Some(spec) = &p1cpy.spec {
            info!("Got blog pod with containers: {:?}", spec.containers);
            assert_eq!(spec.containers[0].name, "blog");
        }

        // Replace its spec
        info!("Patch Pod blog");
        let patch = json!({
            "metadata": {
                "resourceVersion": Meta::resource_ver(&p1cpy),
            },
            "spec": {
                "activeDeadlineSeconds": 5
            }
        });
        println!("{}", patch);
        Ok(())
    }

}

