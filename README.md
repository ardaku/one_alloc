# One Alloc
A custom allocator that allows a singular allocation of a specific size
known ahead of time.

For situations where one allocation is required, but otherwise allocations are
not allowed (usually for constrained embedded devices).  Extremely small (only
32 lines of code).
