# Rust RwLock vs Mutex
Performance testing of RwLocks against Mutexes to test the applicability of [this video](https://www.youtube.com/watch?v=nFtauTYiJlA) for Rust.

# Results
![](/images/5000iters.png)
![](/images/500iters.png)
![](/images/50iters.png)
![](/images/5iters.png)

# Context
Watch this great video by [Software Chats](https://www.youtube.com/@softwarechats9307) for context on the results:
<iframe width="560" height="315" src="https://www.youtube.com/embed/nFtauTYiJlA?si=B6-Wv8rawZpXkKN9" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>