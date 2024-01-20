# Rust RwLock vs Mutex
Performance testing of RwLocks against Mutexes to test the applicability of [this video](https://www.youtube.com/watch?v=nFtauTYiJlA) for Rust.

# Results
![](/images/5000iters.png)
![](/images/500iters.png)
![](/images/50iters.png)
![](/images/5iters.png)

# Context
Watch this great video by [Software Chats](https://www.youtube.com/@softwarechats9307) for context on the results:
<br>
<p style="font-size: 4rem">Is RWMutex slower than Mutex?</p>

[![Is RWMutex slower than Mutex? Youtube Video](http://img.youtube.com/vi/nFtauTYiJlA/0.jpg)](http://www.youtube.com/watch?v=nFtauTYiJlA "Is RWMutex slower than Mutex?")
