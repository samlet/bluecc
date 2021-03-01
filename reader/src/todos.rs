use sqlx::MySqlPool;

pub async fn add_todo(pool: &MySqlPool, description: String) -> anyhow::Result<u64> {
    // Insert the task, then obtain the ID of this row
    let todo_id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ? )
        "#,
        description
    )
        .execute(pool)
        .await?
        .last_insert_id();

    Ok(todo_id)
}

pub async fn complete_todo(pool: &MySqlPool, id: u64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?
        "#,
        id
    )
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn list_todos(pool: &MySqlPool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
        .fetch_all(pool)
        .await?;

    // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
    //       0 = false, non-0 = true
    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done != 0 { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}

