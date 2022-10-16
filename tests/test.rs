use bloom_filter::BloomFilter;

#[test]
fn test_bloom_filter() {
    let n = 8 * 1024 * 1024;
    let m = 1000000;
    let k = 2;
    let mut bloom_filter = BloomFilter::build(n, m, k);
    println!("{:.4}", bloom_filter.false_positive_probability());

    bloom_filter.insert("google");
    bloom_filter.insert("facebook");
    bloom_filter.insert("yandex");

    assert_eq!(bloom_filter.contains("google"), true);
    assert_eq!(bloom_filter.contains("facebook"), true);
    assert_eq!(bloom_filter.contains("yandex"), true);
    assert_eq!(bloom_filter.contains("microsoft"), false);
    assert_eq!(bloom_filter.contains("oracle"), false);
    assert_eq!(bloom_filter.contains("redhat"), false);
}
