# Geektime Rust 语言训练营

## 第四周: Rust 生态系统

![image-20250110122623768](assets/image-20250110122623768.png)

![image-20250110135828332](assets/image-20250110135828332.png)

![image-20250110155102979](assets/image-20250110155102979.png)

![image-20250111114134033](assets/image-20250111114134033.png)

![image-20250111131228716](assets/image-20250111131228716.png)

![image-20250111143306461](assets/image-20250111143306461.png)

![image-20250112102259981](./assets/image-20250112102259981.png)

![image-20250112140251739](./assets/image-20250112140251739.png)

![image-20250113094407853](assets/image-20250113094407853.png)

![image-20250113125021138](assets/image-20250113125021138.png)

> 注:
>
> ```rust
> async fn shorten(
>     State(state): State<AppState>,
>     Json(data): Json<ShortenReq>,
> ) -> Result<Json<ShortenRes>, StatusCode> {
>     todo!()
> }
> ```
>
> 如果在axum的handler中需要访问http body中的内容, 请将其放在最后, 否则会报错

![image-20250113154939524](assets/image-20250113154939524.png)
