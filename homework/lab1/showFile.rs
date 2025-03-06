// use std::fs;
// //use std::path::Path;

// fn main() -> std::io::Result<()>{
//     let entries = fs::read_dir("./")?;
    
//     for entry in entries {
//         let entry = entry?;
//         let path = entry.path();

//         if let Some(file_name) = path.file_name() {
//             println!("{:?}", file_name);
//         }else{
//             println!("No file name");
//         }
//     }
//     Ok(())
// }

#include<iostream>

using namespace std;
int main(){
    int cnt = 0;
    for(long long i=1;i <=1073741823; i ++){
        long long score =0;
        long long num = i;
        while(num >0){
            if(num & 1){
                score += 10;
            }else{
                score =0;
            }
            if (score == 100) break;
            if (score == 70) cnt ++;
            num >>= 1;
        }
    }
    cout << cnt << endl;
    return 0;
}