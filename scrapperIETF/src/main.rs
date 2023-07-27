mod utils;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use url::Url;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use async_recursion::async_recursion;






// async fn traverse_dependencies(dependencies_rfcs: &HashMap<String, Vec<String>>, rfc_number: &str) {
//     match dependencies_rfcs.get(rfc_number) {
//         Some(dependencies) => {
//             println!("Dependencies for {}: {:?}", rfc_number, dependencies);
//             for rfc_related in dependencies {
//                 traverse_dependencies(dependencies_rfcs, rfc_related);
//             }
//         }
//         None => {
//             println!("No data found for the key: {}", rfc_number);
//         }
//     }
// }

#[async_recursion]
async fn search_dependencies_loop(dependencies_rfcs: &mut HashMap<String, Vec<String>>, rfc_number: &String){

    match dependencies_rfcs.get(rfc_number) {
        Some(dependencies) => {
            // 'dependencies' is of type '&Vec<String>'
            println!("Dependencies for {}: {:?}", rfc_number, dependencies);
            
            for rfc_related in dependencies.clone() {

                let url_related = "https://www.ietf.org/rfc/rfc".to_string() + &rfc_related + ".txt";
                search_dependencies(dependencies_rfcs, &url_related ,&rfc_related).await;
                search_dependencies_loop(dependencies_rfcs, &rfc_related).await;

                match dependencies_rfcs.get(&rfc_related){
                    Some(dependencies_related) => {
                        println!("Dependencies related for {}: {:?}", &rfc_related, dependencies_related);
                        // traverse_dependencies(dependencies_rfcs, &rfc_related).await;

                    }
                    None => {
                        println!("None");
                    }
                }
            }
        }
        None => {
            println!("No data found for the key: {}", &rfc_number);
        }
    }
}

async fn search_dependencies(dependencies_rfcs: &mut HashMap<String, Vec<String>>, url_rfc: &String, rfc_number: &String){
    
    let client              = utils::get_client();
    let url                 = url_rfc;
    let result              = client.get(url).send().await.unwrap();
    let html_status         = match result.status() 
                                {
                                    StatusCode::OK => result.text().await.unwrap(),
                                    _ => panic!("Something went wrong"),
                                };


    let re                    = Regex::new(r"RFC(\d{4})").unwrap();
    let mut rfc_numbers:        Vec<String> = Vec::new();
    let mut unique_rfc_numbers: HashSet<String> = HashSet::new();

    for capture in re.captures_iter(&html_status) 
    {
        if let Some(rfc_number_loop) = capture.get(1) 
        {
            if rfc_number_loop.as_str().to_string() != rfc_number.to_string()
            {
                unique_rfc_numbers.insert(rfc_number_loop.as_str().to_string());
            }
        }
    }

    rfc_numbers.extend(unique_rfc_numbers);
    rfc_numbers.dedup();

    dependencies_rfcs.insert(
        rfc_number.clone(),
        rfc_numbers,
    );

    // println!("Dependences for URL with RFC number {}: {:?}", rfc_number, dependencies_rfcs.get(rfc_number));

}

async fn find_number_rfc(url: &String, rfc_number: &mut String ){
    
    let rfc_number_regex    = Regex::new(r"\d{4}").unwrap();
    
    if let Some(captures) = rfc_number_regex.captures(url){
        if let Some(capture_text) = captures.get(0){
            println!("rfc captured: {}", capture_text.as_str());
            *rfc_number = capture_text.as_str().to_string();
            
        } else {
            println!("No rfc number found in the URL");
        }
    }


}

#[tokio::main]
async fn main() {


    let mut dependencies_rfcs: HashMap<String, Vec<String>> = HashMap::new();
    let mut url = "https://www.ietf.org/rfc/rfc6249.txt".to_string();
    let mut rfc_number      = String::new();

    find_number_rfc(&url, &mut rfc_number).await;
    search_dependencies(&mut dependencies_rfcs, &url, &rfc_number).await;
    search_dependencies_loop(&mut dependencies_rfcs, &rfc_number).await;





}
