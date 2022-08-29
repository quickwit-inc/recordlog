use std::ops::Mul;

use crate::multi_record_log;
use crate::MultiRecordLog;

fn read_all_records<'a>(multi_record_log: &'a MultiRecordLog, queue: &str) -> Vec<(u64, &'a [u8])> {
    let mut records = Vec::new();
    let mut next_pos = 0;
    while let Some((pos, payload)) = multi_record_log.get_after(queue, next_pos) {
        records.push((pos, payload));
        next_pos = pos + 1;
    }
    records
}

#[tokio::test]
async fn test_multi_record_log() {
    let tempdir = tempfile::tempdir().unwrap();
    {
        let mut multi_record_log = MultiRecordLog::open(tempdir.path()).await.unwrap();
        multi_record_log
            .append_record("queue1", b"hello")
            .await
            .unwrap();
        multi_record_log
            .append_record("queue2", b"maitre")
            .await
            .unwrap();
        multi_record_log
            .append_record("queue1", b"happy")
            .await
            .unwrap();
        multi_record_log
            .append_record("queue1", b"tax")
            .await
            .unwrap();
        multi_record_log
            .append_record("queue2", b"corbeau")
            .await
            .unwrap();
        assert_eq!(
            &read_all_records(&multi_record_log, "queue1"),
            &[
                (1u64, b"hello".as_slice()),
                (3u64, b"happy".as_slice()),
                (4u64, b"tax".as_slice())
            ]
        );
        assert_eq!(
            &read_all_records(&multi_record_log, "queue2"),
            &[(2u64, b"maitre".as_slice()), (5u64, b"corbeau".as_slice())]
        );
        assert_eq!(multi_record_log.num_files(), 1);
    }
    {
        let mut multi_record_log = MultiRecordLog::open(tempdir.path()).await.unwrap();
        multi_record_log
            .append_record("queue1", b"bubu")
            .await
            .unwrap();
        assert_eq!(
            &read_all_records(&multi_record_log, "queue1"),
            &[
                (1u64, b"hello".as_slice()),
                (3u64, b"happy".as_slice()),
                (4u64, b"tax".as_slice()),
                (6u64, b"bubu".as_slice())
            ]
        );
        assert_eq!(multi_record_log.num_files(), 2);
    }
}
