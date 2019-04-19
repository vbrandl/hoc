# Caching

To prevent calculating the whole stats each time, the `HEAD` and HoC is cached, once it was calculated. If a cached
version is found, current `HEAD` and cached `HEAD` are compared, if they are the same, the cached value is returned,
else only the HoC between the cached `HEAD` and the current `HEAD` is calculated, added to the cached score and the
cache gets updated.
