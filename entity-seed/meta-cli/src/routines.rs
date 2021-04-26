use meta_gen::{SrvDeles, StateGraph};

pub async fn build_status_type_works(status_type: &str) -> meta_gen::Result<()> {
    let mut dele = SrvDeles::new();
    dele.use_default_token().await?;
    // let status_type="ORDER_ITEM_STATUS";
    let mut stg=StateGraph::new();
    let result=stg.build_status_type(&dele, status_type).await?;
    if result {
        stg.draw()?;
    }else{
        println!("cannot build status type: {}", status_type);
    }

    Ok(())
}

