use galois_lib::galois;
use erasure_coding::erasure;

fn test_field_props(){

    for i in 1..=255 {
      assert!(galois::mul(galois::invert(i), i) == 1);
    }
    for i in 0..=255 {
      for j in 0..=255 {
        for k in 0..=255 {
          let a1 = galois::mul(i, galois::mul(j, k));
          let a2 = galois::mul(galois::mul(i, j), k);
          assert!(a1 == a2);
          let b1 = galois::add(i, galois::add(j, k));
          let b2 = galois::add(galois::add(i, j), k);
          assert!(b1 == b2);
        }
      }
    }
    for i in 0..=255 {
      for j in 0..=255 {
        assert!(galois::mul(i, j) == galois::mul(j, i));
        assert!(galois::add(i, j) == galois::add(j, i));
      }
    }
    for i in 0..=255 {
      assert!(galois::mul(i, 1) == i);
      assert!(galois::add(i, 0) == i);
    }
    for i in 0..=255 {
      assert!(galois::add(i, i) == 0);
    }
    
    for i in 0..=255 {
      for j in 0..=255 {
        for k in 0..=255 {
          assert!(galois::mul(i, galois::add(j, k)) == galois::add(galois::mul(i, j), galois::mul(i, k)));
        }
      }
    }
}

fn main() {
  let data: Vec<Vec<u8>> = vec![
    vec![1],
    vec![2],
    vec![3]
  ];

  let encoded = erasure::encode(data.to_vec(), 5);
  let mut recovered: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
  let present: Vec<bool> = vec![false, true, true, false, true];
  recovered.push(encoded[1].to_vec());
  recovered.push(encoded[2].to_vec());
  recovered.push(encoded[4].to_vec());
  let decoded = erasure::decode(recovered.to_vec(), present);
  println!("DATA   : {:?}", data);
  println!("ENCODED: {:?}", encoded);
  println!("DECODED: {:?}", decoded);

}
