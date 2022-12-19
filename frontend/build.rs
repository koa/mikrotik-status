use std::fs;

use anyhow::Result;
use backend::api::create_schema;

fn main() -> Result<()> {
    write_graphql_schema()?;
    Ok(())
}

fn write_graphql_schema() -> Result<()> {
    let schema = create_schema();
    fs::write("graphql/schema.graphql", schema.sdl())?;
    Ok(())
}
