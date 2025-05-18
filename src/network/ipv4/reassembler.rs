use super::header::Header;
use super::parser;
use std::collections::HashMap;

struct Reassembler {
    store: HashMap<u16, Vec<u8>>,
    last_fragment_offset: Option<u16>,
}

pub struct Assorter {
    store: HashMap<u16, Reassembler>,
}

impl Assorter {
    pub fn new() -> Self {
        Assorter {
            store: HashMap::new(),
        }
    }

    pub fn add_and_check(&mut self, packet: &[u8]) -> Option<(Header, Vec<u8>)> {
        let (header, payload) = parser::parse(packet);

        // そのidの2番目以降のフラグメント
        if let Some(reassembler) = self.store.get_mut(&header.id) {
            reassembler.insert(&header, payload.to_vec());
        } else {
            // そのidの最初のフラグメント
            let mut reassembler = Reassembler {
                store: HashMap::new(),
                last_fragment_offset: None,
            };
            reassembler.insert(&header, payload.to_vec());
            self.store.insert(header.id, reassembler);
        }

        let reassembler = self.store.get(&header.id).expect("Reassembler lost");
        if reassembler.has_all_neccessary_fragments() {
            if let Some(reassembler) = reassembler.reassemble() {
                self.store.remove(&header.id);
                return Some((header, reassembler));
            } else {
                panic!("Reassembler lost");
            }
        } else {
            None
        }
    }
}

impl Reassembler {
    pub fn insert(&mut self, header: &Header, payload: Vec<u8>) {
        self.store.insert(header.fragment_offset, payload);
        if header.is_last_fragment() {
            self.last_fragment_offset = Some(header.fragment_offset);
        }
    }
    pub fn has_all_neccessary_fragments(&self) -> bool {
        if self.last_fragment_offset.is_none() {
            return false;
        }
        let mut offset: u16 = 0;
        loop {
            let fragment = self.store.get(&offset);
            match fragment {
                Some(fragment) => {
                    if self.last_fragment_offset == Some(offset) {
                        return true;
                    }
                    offset += fragment.len() as u16;
                }
                None => {
                    return false;
                }
            }
        }
    }

    fn reassemble(&self) -> Option<Vec<u8>> {
        let mut fragments: Vec<(&u16, &Vec<u8>)> = self.store.iter().collect();
        fragments.sort_by_key(|(k, _)| *k);
        let mut result = Vec::new();
        for (_, fragment) in fragments {
            result.extend_from_slice(fragment);
        }
        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テスト用のIPv4パケットを作成する関数
    fn create_test_packet(id: u16, offset: u16, is_last: bool, data: &[u8]) -> Vec<u8> {
        let mut flags: u8 = 0;
        // フラグメントされていない場合はDFフラグを立てる
        if offset == 0 && is_last {
            flags |= 0x40; // Don't Fragment
        }
        // 最後のフラグメントでない場合はMFフラグを立てる
        if !is_last {
            flags |= 0x20; // More Fragments
        }

        // フラグメントオフセット
        let offset_high = (offset >> 8) as u8 & 0x1F; // 上位5ビット
        let offset_low = offset as u8; // 下位8ビット

        // IPv4ヘッダー (20バイト)
        let mut packet = vec![
            0x45,
            0x00, // バージョン、ヘッダ長、ToS
            0x00,
            0x14 + data.len() as u8, // パケット長（ヘッダ + データ長）
            (id >> 8) as u8,
            id as u8, // ID
            flags | offset_high,
            offset_low, // フラグとフラグメントオフセット
            0x40,
            0x11, // TTL, プロトコル(UDP)
            0x00,
            0x00, // ヘッダチェックサム (計算しない)
            192,
            168,
            1,
            1, // 送信元IP
            192,
            168,
            1,
            2, // 宛先IP
        ];

        // データ部分を追加
        packet.extend_from_slice(data);
        packet
    }

    #[test]
    fn test_single_packet() {
        let mut assorter = Assorter::new();
        let data = b"Hello, this is a single packet!";
        let packet = create_test_packet(1234, 0, true, data);

        // パケット追加
        let result = assorter.add_and_check(&packet);

        // 単一パケットなので再構築結果はすぐに返るはず
        assert!(result.is_some());
        assert_eq!(result.unwrap().1, data);
    }

    #[test]
    fn test_multiple_fragments() {
        let mut assorter = Assorter::new();
        let id = 5678;

        // フラグメント1: オフセット0、最後のフラグメントではない
        let data1 = b"First fragment of the message. ";
        let packet1 = create_test_packet(id, 0, false, data1);

        // フラグメント2: オフセット = データ1の長さ、最後のフラグメント
        let data2 = b"This is the last fragment.";
        let packet2 = create_test_packet(id, data1.len() as u16, true, data2);

        // 順番に追加
        let result1 = assorter.add_and_check(&packet1);
        assert!(result1.is_none()); // まだ全フラグメントが揃っていない

        let result2 = assorter.add_and_check(&packet2);
        assert!(result2.is_some());

        // 完全なメッセージを検証
        let expected_data = [&data1[..], &data2[..]].concat();
        assert_eq!(result2.unwrap().1, expected_data);
    }

    #[test]
    fn test_fragments_out_of_order() {
        let mut assorter = Assorter::new();
        let id = 9012;

        // フラグメント1: オフセット0、最後のフラグメントではない
        let data1 = b"First part. ";
        let packet1 = create_test_packet(id, 0, false, data1);

        // フラグメント2: 中間フラグメント
        let data2 = b"Middle part. ";
        let packet2 = create_test_packet(id, data1.len() as u16, false, data2);

        // フラグメント3: 最後のフラグメント
        let data3 = b"Last part.";
        let offset3 = data1.len() as u16 + data2.len() as u16;
        let packet3 = create_test_packet(id, offset3, true, data3);

        // 順番を変えて追加（3,1,2）
        let result3 = assorter.add_and_check(&packet3);
        assert!(result3.is_none()); // まだ全フラグメントが揃っていない

        let result1 = assorter.add_and_check(&packet1);
        assert!(result1.is_none()); // まだ全フラグメントが揃っていない

        let result2 = assorter.add_and_check(&packet2);
        assert!(result2.is_some());

        // 完全なメッセージを検証
        let expected_data = [&data1[..], &data2[..], &data3[..]].concat();
        assert_eq!(result2.unwrap().1, expected_data);
    }

    #[test]
    fn test_multiple_ids() {
        let mut assorter = Assorter::new();

        // ID1のフラグメント
        let id1 = 1111;
        let data1_1 = b"First message, first part. ";
        let packet1_1 = create_test_packet(id1, 0, false, data1_1);
        let data1_2 = b"First message, last part.";
        let packet1_2 = create_test_packet(id1, data1_1.len() as u16, true, data1_2);

        // ID2のフラグメント
        let id2 = 2222;
        let data2_1 = b"Second message, only part.";
        let packet2_1 = create_test_packet(id2, 0, true, data2_1);

        // ID1の最初のフラグメントを追加
        let result = assorter.add_and_check(&packet1_1);
        assert!(result.is_none());

        // ID2の単一フラグメントを追加
        let result = assorter.add_and_check(&packet2_1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().1, data2_1);

        // ID1の最後のフラグメントを追加
        let result = assorter.add_and_check(&packet1_2);
        assert!(result.is_some());
        let expected_data = [&data1_1[..], &data1_2[..]].concat();
        assert_eq!(result.unwrap().1, expected_data);
    }
}
