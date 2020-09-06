use galois_lib::galois;

pub fn transformation_matrix(m: u8, n: u8) -> Vec<Vec<u8>> {
  let mut res = Vec::<Vec<u8>>::new();
  for i in 0..m as usize {
    res.push(vec![1]);
    for j in 1..n as usize {
      let v = res[i][j - 1];
      res[i].push(galois::mul(v, i as u8));
    }
  }
  let mut to_be_inv: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
  
  for i in 0..n as usize {
    to_be_inv.push(res[i].to_vec());
  }

  // res.push(Vec::<u8>::new());
  // for i in 0..m as usize {
  //   res[0].push(1);
  // }
  // let mut to_be_inv: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
  // for i in 1..m as usize {
  //   res.push(Vec::<u8>::new());
  //   for j in 0..n as usize {
  //     let v = res[i - 1][j];
  //     res[i].push(galois::mul(v, j as u8));
  //   }
  //   if i == (n - 1) as usize {
  //     to_be_inv = res.to_vec();
  //   }
  // }
  return galois::mul_mat(res, galois::invert_mat(to_be_inv));
}

pub fn encode(a: Vec<Vec<u8>>, m: u8) -> Vec<Vec<u8>> {
  let mat = transformation_matrix(m, a.len() as u8);
  return galois::mul_mat(mat, a);
}

pub fn decode(a: Vec<Vec<u8>>, present: Vec<bool>) -> Vec<Vec<u8>> { 
  let n: u8 = a.len() as u8;
  let m: u8 = present.len() as u8;
  let mat = transformation_matrix(m, n);
  let mut map: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
  for i in 0..m as usize {
    if present[i] {
      map.push(mat[i].to_vec());
    }
  }
  map = galois::invert_mat(map);
  return galois::mul_mat(map, a.to_vec());
}
