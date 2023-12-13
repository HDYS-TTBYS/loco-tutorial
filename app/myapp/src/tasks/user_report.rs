use std::collections::BTreeMap;

use loco_rs::prelude::*;
use crate::models::_entities::users;

#[allow(clippy::module_name_repetitions)]
pub struct UserReport;
#[async_trait]
impl Task for UserReport {
    fn task(&self) -> TaskInfo {
      // description that appears on the CLI
        TaskInfo {
            name: "user_report".to_string(),
            detail: "output a user report".to_string(),
        }
    }

    // variables through the CLI: 
    // `$ cargo loco task name:foobar count:2` 
    // will appear as {"name":"foobar", "count":2} in `vars`
    async fn run(&self, app_context: &AppContext, vars: &BTreeMap<String, String>) -> Result<()> {
        let users = users::Entity::find().all(&app_context.db).await?;
        println!("args: {vars:?}");
        println!("!!! user_report: listing users !!!");
        println!("------------------------");
        for user in &users {
            println!("user: {}", user.email);
        }
        println!("done: {} users", users.len());
        Ok(())
    }
}