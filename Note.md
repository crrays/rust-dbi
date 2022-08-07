1. 函数不支持重载
2. 注意所有权
3. 实现了 fmt::Display 以后，会隐式实现 to_string()->String，所以不需要自己再写一个to_string；