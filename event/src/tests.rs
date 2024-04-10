#[test]
fn deserialize_timer_event() {
    use crate::Event;
    let body = r#"
    {
        "messages": [
          {
            "event_metadata": {
              "event_id": "a1s41g2n5g0o********",
              "event_type": "yandex.cloud.events.serverless.triggers.TimerMessage",
              "created_at": "2019-12-04T12:05:14.227761Z",
              "cloud_id": "b1gvlrnlei4l********",
              "folder_id": "b1g88tflru0e********"
            },
            "details": {
              "trigger_id": "a1sfe084v4se********",
              "payload": "payload-message"
            }
          }
        ]
      }
      
    "#;

    let event = serde_json::from_str::<Event>(body);
    if let Err(e) = &event {
        println!("{:?}", e);
    }
    assert!(event.is_ok());
}

#[test]
fn deserialize_queue_message() {
    use crate::Event;
    let body = r#"
    {
        "messages":[
           {
              "event_metadata":{
                 "event_id":"cce76685-5828-4304-a83d-9564********",
                 "event_type":"yandex.cloud.events.messagequeue.QueueMessage",
                 "created_at":"2019-09-24T00:54:28.980441Z",
                 "cloud_id":"b1gvlrnlh2sd********",
                 "folder_id":"b1g88tflh2sd********"       
              },
              "details":{
                 "queue_id":"yrn:yc:ymq:ru-central1:21i6v06sqmsa********:event-queue",
                 "message":{
                    "message_id":"cce76685-5828-4304-a83d-9564********",
                    "md5_of_body":"d29343907090dff4cec4a9a0********",
                    "body":"message body",
                    "attributes":{
                       "SentTimestamp":"1569285804456"
                    },
                    "message_attributes":{
                       "messageAttributeKey":{
                          "data_type":"StringValue",
                          "string_value":"value"
                       }
                    },
                    "md5_of_message_attributes":"83eb2d0afefb150c1ffe69f6********"
                 }
              }
           },
           {
              "event_metadata":{
                 "event_id":"1f32fd25-11fc-4c08-88e7-d871********",
                 "event_type":"yandex.cloud.events.messagequeue.QueueMessage",
                 "created_at":"2019-09-24T00:54:28.980492Z",
                 "cloud_id":"b1gvlrnlh2sd********",
                 "folder_id":"b1g88tflh2sd********"
              },
              "details":{
                 "queue_id":"yrn:yc:ymq:ru-central1:21i6v06sqmsa********:event-queue",
                 "message":{
                    "message_id":"1f32fd25-11fc-4c08-88e7-d871********",
                    "md5_of_body":"d29343907090dff4cec4a9a0********",
                    "body":"message body",
                    "attributes":{
                       "SentTimestamp":"1569285806456"
                    },
                    "message_attributes":{
                       "messageAttributeKey":{
                          "data_type":"StringValue",
                          "string_value":"value"
                       }
                    },
                    "md5_of_message_attributes":"83eb2d0afefb150c1ffe69f6********"
                 }
              }
           }
        ]
     }
     
    "#;

    let event = serde_json::from_str::<Event>(body);
    if let Err(e) = &event {
        println!("{:?}", e);
    }
    assert!(event.is_ok());
}
