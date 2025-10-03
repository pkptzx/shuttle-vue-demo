use notion_client::{
    endpoints::{
        databases::query::request::{self, QueryDatabaseRequest, Sort, SortDirection, Timestamp},
        Client,
    },
    objects::{
        block::{Block, TableRowsValue},
        rich_text::{Annotations, RichText, Text},
    },
};

#[tokio::test]
async fn test_notion_append_list() {
    let NOTION_TOKEN = "0";
    let block_id = "271b789d-5abe-80af-b974-d85571437f6d";
    let client = Client::new(NOTION_TOKEN.to_string(), None);
    let Ok(client) = client else {
        panic!("error");
    };

    // Set up request parameters
    let mut request_builder = notion_client::endpoints::blocks::append::request::AppendBlockChildrenRequestBuilder::default();
    let tableRowsValue = TableRowsValue {
        cells: vec![
            vec![RichText::Text {
                text: Text {
                    content: "6".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: Some("6".to_string()),
                href: None,
            }],
            vec![RichText::Text {
                text: Text {
                    content: "7".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: Some("7".to_string()),
                href: None,
            }],
            vec![RichText::Text {
                text: Text {
                    content: "8".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: Some("8".to_string()),
                href: None,
            }],
        ],
    };
    let request = request_builder
        .children(vec![Block {
            block_type: notion_client::objects::block::BlockType::TableRow {
                table_row: tableRowsValue,
            },
            object: Some("block".to_string()),
            id: None,
            parent: None,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: None,
            has_children: None,
        }])
        .build()
        .unwrap();

    // Send request
    let res = client.blocks.append_block_children(block_id, request).await;
    match res {
        Ok(r) =>  print!("result: {:#?}", r),
        Err(e) =>  print!("error: {:#?}", e),
    }

    // See result 271b789d-5abe-8162-849d-e81b0325d497
    // print!("{:#?}", res);
}


#[tokio::test]
async fn test_notion_update_list() {
    let NOTION_TOKEN = "0";
    let block_id = "0";
    let client = Client::new(NOTION_TOKEN.to_string(), None);
    let Ok(client) = client else {
        panic!("error");
    };

    // Set up request parameters
    let mut request_builder = notion_client::endpoints::blocks::update::request::UpdateABlockRequestBuilder::default();
     
    let tableRowsValue = TableRowsValue {
        cells: vec![
            vec![RichText::Text {
                text: Text {
                    content: "好".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: Some("好".to_string()),
                href: None,
            }],
            vec![RichText::Text {
                text: Text {
                    content: "0".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: Some("0".to_string()),
                href: None,
            }],
            vec![RichText::Text {
                text: Text {
                    content: "9".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: Some("9".to_string()),
                href: None,
            }],
        ],
    };
    let request = request_builder.archived(false).block(

    Block {
            block_type: notion_client::objects::block::BlockType::TableRow {
                table_row: tableRowsValue,
            },
            object: Some("block".to_string()),
            id: None,
            parent: None,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: None,
            has_children: None,
        })
        .build()
        .unwrap();

    // Send request
    let res = client.blocks.update_a_block(block_id, request).await;
    match res {
        Ok(r) =>  print!("result: {:#?}", r),
        Err(e) =>  print!("error: {:#?}", e),
    }

    // See result 271b789d-5abe-8162-849d-e81b0325d497
    // print!("{:#?}", res);
}
