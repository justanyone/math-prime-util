//! vec: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use vec::*`.
// MPU sort.c: _mid3_iv_val, _mid2_of_4_iv, _partition_iv, _qs_iv.
fn median(a: i64,b: i64,c: i64)->i64 { let s=[a,b,c]; let x=a>b; let y=a>c; let z=b>c; s[usize::from(x==y)+usize::from(y^z)] }
fn middle_four(a: &mut [i64]) { if a[0]>a[1] {a.swap(0,1);} if a[2]>a[3] {a.swap(2,3);} a[2]=a[0].max(a[2]);a[1]=a[1].min(a[3]); }
fn quicksort(a: &mut [i64], mut bad: u32) {
 let n=a.len(); if n<=16 { for i in 1..n {let t=a[i];let mut j=i;while j>0 && a[j-1]>t {a[j]=a[j-1];j-=1;}a[j]=t;}return; }
 let pivot=if n<=40 {median(a[0],a[(n-1)/2],a[n-1])} else {let mut s=[0;9];for i in 0..9 {s[i]=a[i*((n-1)/8)];}middle_four(&mut s);middle_four(&mut s[4..]);s[0]=s[5];s[3]=s[8];middle_four(&mut s);median(s[6],s[1],s[2])};
 let (mut i,mut j)=(0,n-1);loop {while a[i]<pivot {i+=1;}while a[j]>pivot {j-=1;}if i>=j {break;}a.swap(i,j);i+=1;j-=1;}
 let split=j+1;if split<n/8 || n-split<n/8 {bad=bad.saturating_sub(1);if bad==0 {heapsort(a);return;}}
 let (left,right)=a.split_at_mut(split);quicksort(left,bad);quicksort(right,bad);
}
// MPU sort.c: _heapsort.
fn heapsort(a: &mut [i64]) { let mut start=a.len()/2;if start==0 {return;}let mut end=a.len()-1;loop {let r=if start>0 {start-=1;a[start]} else if end>0 {let r=a[end];a[end]=a[0];end-=1;r} else {return;};let mut b=start;let mut c=2*b+1;whil <<cut 120 chars by kit>>
// MPU sort.c: sort_iv_array and _radixsort (byte passes skip constant digits).
fn sort_signed(a: &mut [i64]) {
 if a.len()<400 {if a.len()>1 {quicksort(a,a.len().ilog2());}return;}
 let first=a[0];let mut vary=0_u64;let mut width=0_u64;let mut asc=true;let mut desc=true;let mut prev=first;
 for &x in a.iter() {let bits=u64::from_ne_bytes(x.to_ne_bytes());vary|=bits^u64::from_ne_bytes(first.to_ne_bytes());width|=if x<0 {!bits} else {bits};asc&=prev<=x;desc&=prev>=x;prev=x;}
 if asc {return;}if desc {a.reverse();return;}
 let mut signshift=0;while signshift<56 && width>>(signshift+7)!=0 {signshift+=8;}
 let mut buffer=Vec::new();if buffer.try_reserve_exact(a.len()).is_err() {quicksort(a,a.len().ilog2());return;}buffer.resize(a.len(),0);
 for shift in (0..=signshift).step_by(8) {if vary>>shift & 255==0 {continue;}let sign=if shift==signshift {128} else {0};let key=|x:i64| -> usize {usize::from(x.to_le_bytes()[shift/8])^sign};let mut counts=[0_usize;256];for &x in a.iter() { <<cut 180 chars by kit>>
}

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 250 bytes)
