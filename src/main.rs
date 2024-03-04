// Copyright 2023-2024 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use laserfiche_rs::laserfiche;
use serde_json::json;


fn main() {
    let api = laserfiche::blocking::LFApiServer{address: format!("<SERVER_ADDRESS>"), repository: format!("<REPOSITORY>")};
    let auth = laserfiche::blocking::Auth::new(api.clone(), format!("<USER>"), format!("<PASSWORD>")).unwrap();

    match auth{
        laserfiche::blocking::AuthOrError::Auth(ath) => {

            // New folder test
            // let entry = laserfiche::blocking::Entry::new_path(api.clone(), ath.clone(), "incoming".to_string(), "".to_string(), 1).unwrap();
            // match entry{
            //     laserfiche::blocking::EntryOrError::Entry(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::EntryOrError::LFAPIError(err) => println!("{:?}", err),
            // }

            // // Import file test
            // let entry = laserfiche::blocking::Entry::import(api.clone(), ath.clone(), "incoming".to_string(), "test.png".to_string(), 1).unwrap();
            // match entry{
            //     laserfiche::blocking::ImportResultOrError::ImportResult(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::ImportResultOrError::LFAPIError(err) => println!("{:?}", err),
            // }

            // Export file test
            // let entry = laserfiche::blocking::Entry::export(api.clone(), ath.clone(), 5, "export_test.png").unwrap();
            // match entry{
            //     laserfiche::blocking::BitsOrError::Bits(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::BitsOrError::LFAPIError(err) => println!("{:?}", err),
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
            // let entry = laserfiche::blocking::Entry::update_metadata(api.clone(), ath.clone(), 5, test_dat).unwrap();
            // match entry{
            //     laserfiche::blocking::MetadataResultOrError::Metadata(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::MetadataResultOrError::LFAPIError(err) => println!("{:?}", err),
            // }

            // let entry = laserfiche::blocking::Entry::get_metadata(api.clone(), ath.clone(), 9).unwrap();
            // match entry{
            //     laserfiche::blocking::MetadataResultOrError::Metadata(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::MetadataResultOrError::LFAPIError(err) => println!("{:?}", err),
            // }

            // Delete entry test
            // let entry = laserfiche::blocking::Entry::delete(api.clone(), ath.clone(), 9, "idk".to_string()).unwrap();
            // match entry{
            //     laserfiche::blocking::LFObject::DeletedObject(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }

            // // Rename entry test
            // let entry = laserfiche::blocking::Entry::patch(api.clone(), ath.clone(), 20, None, Some("idk".to_string())).unwrap();
            // match entry{
            //     laserfiche::blocking::LFObject::Entry(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }
            
            // // Move entry test
            // let entry = laserfiche::blocking::Entry::patch(api.clone(), ath.clone(), 20, Some(21), None).unwrap();
            // match entry{
            //     laserfiche::blocking::LFObject::Entry(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }
                       
            // // Get fields test
            // let entry = laserfiche::blocking::Entry::get_field(api.clone(), ath.clone(), 21, 5).unwrap();
            // match entry{
            //     laserfiche::blocking::LFObject::Field(ent) => {
            //         println!("{:?}", ent);
            //     },
            //     laserfiche::blocking::LFObject::LFAPIError(err) => println!("{:?}", err),
            //     _ => {}
            // }
            
            // list folders test
            let entry = laserfiche::blocking::Entry::list(api.clone(), ath.clone(), 1).unwrap();
            match entry{
                laserfiche::blocking::EntriesOrError::Entries(ent) => {
                    println!("{:?}", ent);
                },
                laserfiche::blocking::EntriesOrError::LFAPIError(err) => println!("{:?}", err),
            }

        },
        laserfiche::blocking::AuthOrError::LFAPIError(err) => println!("{:?}", err),
    }
    
}
