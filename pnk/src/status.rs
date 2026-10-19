use docker_wrapper::{DockerCommand, PsCommand};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn status(container_name: &Option<String>) -> Result<()> {
    let name = container_name.as_deref();
    match name {
        None => {
            let ps_cmd = PsCommand::new()
                .format_template("table {{.ID}}\\t{{.Names}}\\t{{.Status}}\\t{{.Ports}}")
                .execute()
                .await?;
            println!("{}", ps_cmd.stdout);
            Ok(())
        }

        Some(name) => {
            let ps_cmd = PsCommand::new()
                .filter(format!("name={}", name))
                .format_template("table {{.ID}}\\t{{.Names}}\\t{{.Status}}\\t{{.Ports}}")
                .execute()
                .await?;
            println!("{}", ps_cmd.stdout);
            Ok(())
        }
    }
}
