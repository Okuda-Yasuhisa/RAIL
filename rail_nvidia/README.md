# RAIL NVIDIA

RAIL NVIDIA は、RAIL Nacro が提供する属性マクロが付与された関数を ptx へコンパイルする機能を提供します。

## 使用例

```rust
use rail_nuntime::{cuda_runner, CudaErr};
use rail_nacro::nvidia::{Global, CudaOnly};

#[derive(Global)]
pub struct CudaGlobalStruct<T, R> {
    // この関数の引数
    args: Vec<T>,
    // この関数の戻り値
    return_type: Result<R, CudaErr>,
    // この関数本体
    exe_fn: fn() -> (),
}

#[derive(CudaOnly)]
pub struct CudaOnlyStruct<T, R> {
    // この関数の引数
    args: Vec<T>,
    // この関数の戻り値
    return_type: Result<R, CudaErr>,
    // この関数本体
    exe_fn: fn() -> ()
}

impl<T, R> CudaGlobalStruct<T, R> {
    pub fn new() -> Self {
        CudaStruct {
            args: Vec::new(),
            return_type: Default::default(),
            exe_fn: || {},
        }
    }

    pub fn set_args(&mut self, args: Vec<T>) {
        self.args = args;
    }

    pub fn set_fn(&mut self, func: fn() -> ()) {
        self.exe_fn = func;
    }

    pub fn execute(&self) -> &R {
        // ここでCUDAの実行処理を行う
        // cuda_runnerを使用して実際の処理を実装
        // 例: cuda_runner::execute(self.is_global, &self.args, self.exe_fn)

        &self.return_type
    }

    pub fn get_result(&self) -> &R {
        &self.return_type
    }
}

fn main() {
    let vec_a = vec![1, 2, 3, 4];
    let vec_b = vec![5, 6, 7, 8];

    let mut cuda_struct = CudaStruct::<Vec<i32>, Vec<i32>>::new();
    cuda_struct.set_args(vec![vec_a, vec_b]);

    // CUDA上で実行する関数を定義
    cuda_struct.set_fn(|| {
        // ここにCUDA上で実行したい処理を記述
        println!("CUDAで処理実行中...");
    });

    // 実行して結果を取得
    let result = cuda_struct.execute();
    // または
    let result = cuda_struct.get_result();
}
```
