use anyhow::Result;

use crate::data::activity;
use crate::data::bartib_file;
use crate::data::getter;
use crate::view::list;

// lists all currently runninng activities.
pub fn list_running(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let running_activities = getter::get_running_activities(&file_content);

    list::list_running_activities(&running_activities);

    Ok(())
}

// lists tracked activities
//
// the activities will be ordered chronologically.
pub fn list(file_name: &str, filter: getter::ActivityFilter, do_group_activities: bool) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let activities = getter::get_activities(&file_content);
    let mut filtered_activities: Vec<&activity::Activity> =
        getter::filter_activities(activities, &filter).collect();

    filtered_activities.sort_by_key(|activity| activity.start);

    let first_element = filtered_activities.len().saturating_sub(filter.number_of_activities.unwrap_or(filtered_activities.len()));

    if do_group_activities {
        list::list_activities_grouped_by_date(
            &filtered_activities[first_element..],
        );
    } else {
        let with_start_dates = filter.date.is_none();
        list::list_activities(
            &filtered_activities[first_element..],
            with_start_dates,
        );
    }

    Ok(())
}

// prints all errors that occured when reading the bartib file
pub fn check(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let number_of_errors = file_content.iter()
        .filter(|line| line.activity.is_err())
        .count();

    if number_of_errors == 0 {
        println!("All lines in the file have been successfully parsed as activities.");
        return Ok(());
    }

    println!("Found {} line(s) with parsing errors", number_of_errors);

    file_content.iter()
        .filter(|line| line.activity.is_err() && line.plaintext.is_some())
        .for_each(|line| {
            if let Err(e) = &line.activity {
                println!("\n{}\n  -> {} (Line: {})", line.plaintext.as_ref().unwrap(), e.to_string(), line.line_number.unwrap_or(0));
            }
        });

    Ok(())
}

// lists all projects
pub fn list_projects(file_name: &str, current: bool) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let mut all_projects: Vec<&String> = getter::get_activities(&file_content)
        .filter(|activity| !(current && activity.is_stopped()))
        .map(|activity| &activity.project)
        .collect();

    all_projects.sort_unstable();
    all_projects.dedup();

    for project in all_projects {
        println!("\"{}\"", project);
    }

    Ok(())
}

// return last finished activity
pub fn list_last_activities(file_name: &str, number: usize) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let descriptions_and_projects : Vec<(&String, &String)> = getter::get_descriptions_and_projects(&file_content);
    let first_element = descriptions_and_projects.len().saturating_sub(number);

    list::list_descriptions_and_projects(&descriptions_and_projects[first_element..]);

    Ok(())
}