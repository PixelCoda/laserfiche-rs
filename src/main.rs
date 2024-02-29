// Copyright 2023-2024 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use laserfiche_rs::laserfiche;
use serde_json::json;

#[tokio::main]
async fn main() {
    let api = laserfiche::LFApiServer{address: format!("<SERVER_ADDRESS>"), repository: format!("<REPOSITORY>")};
    let auth = laserfiche::Auth::new(api.clone(), format!("<USER>"), format!("<PASSWORD>")).await.unwrap();


    match auth{
        laserfiche::AuthOrError::Auth(ath) => {

            // New folder test
            // let entry = laserfiche::Entry::new_path(api.clone(), ath.clone(), "incoming".to_string(), "".to_string(), 1).await.unwrap();
            // match entry{
            //     laserfiche::EntryOrError::Entry(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::EntryOrError::LFAPIError(err) => println!("{:?}", err),
            // }

            // // Import file test
            // let entry = laserfiche::Entry::import(api.clone(), ath.clone(), "incoming".to_string(), "test.png".to_string(), 1).await.unwrap();
            // match entry{
            //     laserfiche::ImportResultOrError::ImportResult(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::ImportResultOrError::LFAPIError(err) => println!("{:?}", err),
            // }


            // Export file test
            // let entry = laserfiche::Entry::export(api.clone(), ath.clone(), 5, "export_test.png").await.unwrap();
            // match entry{
            //     laserfiche::BitsOrError::Bits(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::BitsOrError::LFAPIError(err) => println!("{:?}", err),
            // }




            // let test_dat = json!({
            //     "TestField":{
            //       "values":[
            //         {
            //           "value":"fhgfhfghgfhfghfgh",
            //           "position":1
            //         }
            //       ]
            //     }
            // });
            // let entry = laserfiche::Entry::update_metadata(api.clone(), ath.clone(), 5, test_dat).await.unwrap();
            // match entry{
            //     laserfiche::MetadataResultOrError::Metadata(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::MetadataResultOrError::LFAPIError(err) => println!("{:?}", err),
            // }




            let entry = laserfiche::Entry::get_metadata(api.clone(), ath.clone(), 9).await.unwrap();
            match entry{
                laserfiche::MetadataResultOrError::Metadata(ent) => {
                    println!("{:?}", ent);
                },
                laserfiche::MetadataResultOrError::LFAPIError(err) => println!("{:?}", err),
            }

            

            // Delete entry test
            // let entry = laserfiche::Entry::delete(api.clone(), ath.clone(), 9, "idk".to_string()).await.unwrap();
            // match entry{
            //     laserfiche::LFObject::DeletedObject(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }
            

            // // Rename entry test
            // let entry = laserfiche::Entry::patch(api.clone(), ath.clone(), 20, None, Some("idk".to_string())).await.unwrap();
            // match entry{
            //     laserfiche::LFObject::Entry(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }
            
            // // Move entry test
            // let entry = laserfiche::Entry::patch(api.clone(), ath.clone(), 20, Some(21), None).await.unwrap();
            // match entry{
            //     laserfiche::LFObject::Entry(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }
                       
            // // Get fields test
            // let entry = laserfiche::Entry::get_field(api.clone(), ath.clone(), 21, 5).await.unwrap();
            // match entry{
            //     laserfiche::LFObject::Field(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }
            

            // list folders test
            // let entry = laserfiche::Entry::list(api.clone(), ath.clone(), 1).await.unwrap();
            // match entry{
            //     laserfiche::EntriesOrError::Entries(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::EntriesOrError::LFAPIError(err) => println!("{:?}", err),
            // }



        },
        laserfiche::AuthOrError::LFAPIError(err) => println!("{:?}", err),
    }
    
}
