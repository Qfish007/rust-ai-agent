
# 项目说明

## 获取环境变量

```javascript
let url = env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL environment variable not set");
println!("{}", url);
```