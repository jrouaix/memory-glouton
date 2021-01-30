

### Some problem appen with `bastion` ?

```bash
cargo run
```

### 2 seconds mem leaks check

```sh
cargo build
valgrind --leak-check=yes --log-file="valgrind.log" ./target/debug/glouton
```

```text
==373257== Memcheck, a memory error detector
==373257== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==373257== Using Valgrind-3.16.1 and LibVEX; rerun with -h for copyright info
==373257== Command: ./target/debug/glouton
==373257== Parent PID: 360114
==373257== 
==373257== 
==373257== HEAP SUMMARY:
==373257==     in use at exit: 6,396,845 bytes in 49,349 blocks
==373257==   total heap usage: 54,438 allocs, 5,089 frees, 6,543,464 bytes allocated
==373257== 
==373257== 24 bytes in 1 blocks are possibly lost in loss record 18 of 129
==373257==    at 0x483C7F3: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x32C42B: alloc::alloc::alloc (alloc.rs:84)
==373257==    by 0x32C4E9: alloc::alloc::Global::alloc_impl (alloc.rs:164)
==373257==    by 0x32CCC9: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:224)
==373257==    by 0x32C38C: alloc::alloc::exchange_malloc (alloc.rs:314)
==373257==    by 0x3244B9: alloc::sync::Arc<T>::new (sync.rs:330)
==373257==    by 0x320BD2: lever::sync::atomics::AtomicBox<T>::alloc_from (atomics.rs:25)
==373257==    by 0x320B82: lever::sync::atomics::AtomicBox<T>::new (atomics.rs:19)
==373257==    by 0x32B832: lever::txn::transact::TxnManager::txn_build (transact.rs:468)
==373257==    by 0x1939FD: lever::table::lotable::LOTable<K,V,S>::with_capacity_and_hasher (lotable.rs:62)
==373257==    by 0x193633: lever::table::lotable::LOTable<K,V>::with_capacity (lotable.rs:47)
==373257==    by 0x193652: lever::table::lotable::LOTable<K,V>::new (lotable.rs:43)
==373257== 
==373257== 64 bytes in 1 blocks are possibly lost in loss record 44 of 129
==373257==    at 0x483C7F3: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x26C4EB: alloc::alloc::alloc (alloc.rs:84)
==373257==    by 0x26C5A9: alloc::alloc::Global::alloc_impl (alloc.rs:164)
==373257==    by 0x270229: <alloc::alloc::Global as core::alloc::AllocRef>::alloc (alloc.rs:224)
==373257==    by 0x26C44C: alloc::alloc::exchange_malloc (alloc.rs:314)
==373257==    by 0x17BDED: alloc::sync::Arc<T>::new (sync.rs:330)
==373257==    by 0x214FF2: lever::sync::atomics::AtomicBox<T>::alloc_from (atomics.rs:25)
==373257==    by 0x215B62: lever::sync::atomics::AtomicBox<T>::new (atomics.rs:19)
==373257==    by 0x193A78: lever::table::lotable::LOTable<K,V,S>::with_capacity_and_hasher (lotable.rs:71)
==373257==    by 0x193633: lever::table::lotable::LOTable<K,V>::with_capacity (lotable.rs:47)
==373257==    by 0x193652: lever::table::lotable::LOTable<K,V>::new (lotable.rs:43)
==373257==    by 0x270D6D: bastion::dispatcher::GlobalDispatcher::new (dispatcher.rs:278)
==373257== 
==373257== 288 bytes in 1 blocks are possibly lost in loss record 69 of 129
==373257==    at 0x483ED99: calloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x4014F2B: calloc (rtld-malloc.h:44)
==373257==    by 0x4014F2B: allocate_dtv (dl-tls.c:366)
==373257==    by 0x4014F2B: _dl_allocate_tls (dl-tls.c:612)
==373257==    by 0x488A2E5: allocate_stack (allocatestack.c:624)
==373257==    by 0x488A2E5: pthread_create@@GLIBC_2.2.5 (pthread_create.c:634)
==373257==    by 0x392016: std::sys::unix::thread::Thread::new (thread.rs:50)
==373257==    by 0x2CCE91: std::thread::Builder::spawn_unchecked (mod.rs:498)
==373257==    by 0x2CFA4D: std::thread::Builder::spawn (mod.rs:381)
==373257==    by 0x2DAC3A: bastion_executor::thread_manager::DynamicPoolManager::initialize (thread_manager.rs:227)
==373257==    by 0x2EB066: bastion_executor::pool::POOL::{{closure}} (pool.rs:180)
==373257==    by 0x2EE784: core::ops::function::FnOnce::call_once (function.rs:227)
==373257==    by 0x2EE911: core::ops::function::FnOnce::call_once (function.rs:227)
==373257==    by 0x2FD30B: once_cell::sync::Lazy<T,F>::force::{{closure}} (lib.rs:1001)
==373257==    by 0x2FD403: once_cell::sync::OnceCell<T>::get_or_init::{{closure}} (lib.rs:834)
==373257== 
==373257== 576 bytes in 2 blocks are possibly lost in loss record 76 of 129
==373257==    at 0x483ED99: calloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x4014F2B: calloc (rtld-malloc.h:44)
==373257==    by 0x4014F2B: allocate_dtv (dl-tls.c:366)
==373257==    by 0x4014F2B: _dl_allocate_tls (dl-tls.c:612)
==373257==    by 0x488A2E5: allocate_stack (allocatestack.c:624)
==373257==    by 0x488A2E5: pthread_create@@GLIBC_2.2.5 (pthread_create.c:634)
==373257==    by 0x392016: std::sys::unix::thread::Thread::new (thread.rs:50)
==373257==    by 0x2CE4BD: std::thread::Builder::spawn_unchecked (mod.rs:498)
==373257==    by 0x2CF97C: std::thread::Builder::spawn (mod.rs:381)
==373257==    by 0x2DAE49: bastion_executor::thread_manager::DynamicPoolManager::initialize::{{closure}} (thread_manager.rs:201)
==373257==    by 0x2CFD8C: core::iter::traits::iterator::Iterator::for_each::call::{{closure}} (iterator.rs:675)
==373257==    by 0x2FA7CB: core::iter::traits::iterator::Iterator::fold (iterator.rs:2023)
==373257==    by 0x2FA944: core::iter::traits::iterator::Iterator::for_each (iterator.rs:678)
==373257==    by 0x2DA6EF: bastion_executor::thread_manager::DynamicPoolManager::initialize (thread_manager.rs:199)
==373257==    by 0x2EB066: bastion_executor::pool::POOL::{{closure}} (pool.rs:180)
==373257== 
==373257== 8,640 bytes in 30 blocks are possibly lost in loss record 84 of 129
==373257==    at 0x483ED99: calloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x4014F2B: calloc (rtld-malloc.h:44)
==373257==    by 0x4014F2B: allocate_dtv (dl-tls.c:366)
==373257==    by 0x4014F2B: _dl_allocate_tls (dl-tls.c:612)
==373257==    by 0x488A2E5: allocate_stack (allocatestack.c:624)
==373257==    by 0x488A2E5: pthread_create@@GLIBC_2.2.5 (pthread_create.c:634)
==373257==    by 0x392016: std::sys::unix::thread::Thread::new (thread.rs:50)
==373257==    by 0x2CD5E8: std::thread::Builder::spawn_unchecked (mod.rs:498)
==373257==    by 0x2CFAC7: std::thread::Builder::spawn (mod.rs:381)
==373257==    by 0x2DB0A9: bastion_executor::thread_manager::DynamicPoolManager::initialize::{{closure}} (thread_manager.rs:215)
==373257==    by 0x2CFCCC: core::iter::traits::iterator::Iterator::for_each::call::{{closure}} (iterator.rs:675)
==373257==    by 0x2FA6FB: core::iter::traits::iterator::Iterator::fold (iterator.rs:2023)
==373257==    by 0x2FA9D4: core::iter::traits::iterator::Iterator::for_each (iterator.rs:678)
==373257==    by 0x2DABA8: bastion_executor::thread_manager::DynamicPoolManager::initialize (thread_manager.rs:213)
==373257==    by 0x2EB066: bastion_executor::pool::POOL::{{closure}} (pool.rs:180)
==373257== 
==373257== 116,116 bytes in 1,001 blocks are possibly lost in loss record 116 of 129
==373257==    at 0x483C7F3: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x26C4EB: alloc::alloc::alloc (alloc.rs:84)
==373257==    by 0x2897B6: hashbrown::raw::RawTable<T>::new_uninitialized (mod.rs:411)
==373257==    by 0x28AD76: hashbrown::raw::RawTable<T>::fallible_with_capacity (mod.rs:440)
==373257==    by 0x293C92: hashbrown::raw::RawTable<T>::resize (mod.rs:873)
==373257==    by 0x27E513: hashbrown::raw::RawTable<T>::reserve_rehash (mod.rs:754)
==373257==    by 0x295041: hashbrown::raw::RawTable<T>::reserve (mod.rs:707)
==373257==    by 0x2904D6: hashbrown::raw::RawTable<T>::insert (mod.rs:926)
==373257==    by 0x2685A3: hashbrown::map::HashMap<K,V,S>::insert (map.rs:991)
==373257==    by 0x188F99: std::collections::hash::map::HashMap<K,V,S>::insert (map.rs:841)
==373257==    by 0x1EDE0F: bastion::broadcast::Broadcast::register (broadcast.rs:106)
==373257==    by 0x255DD3: bastion::children::Children::launch_child (children.rs:909)
==373257== 
==373257== 148,148 bytes in 1,001 blocks are possibly lost in loss record 119 of 129
==373257==    at 0x483C7F3: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x26C4EB: alloc::alloc::alloc (alloc.rs:84)
==373257==    by 0x287FB6: hashbrown::raw::RawTable<T>::new_uninitialized (mod.rs:411)
==373257==    by 0x28A4A6: hashbrown::raw::RawTable<T>::fallible_with_capacity (mod.rs:440)
==373257==    by 0x291F82: hashbrown::raw::RawTable<T>::resize (mod.rs:873)
==373257==    by 0x27EB43: hashbrown::raw::RawTable<T>::reserve_rehash (mod.rs:754)
==373257==    by 0x294DC1: hashbrown::raw::RawTable<T>::reserve (mod.rs:707)
==373257==    by 0x28F5A6: hashbrown::raw::RawTable<T>::insert (mod.rs:926)
==373257==    by 0x267DCB: hashbrown::map::HashMap<K,V,S>::insert (map.rs:991)
==373257==    by 0x188F4E: std::collections::hash::map::HashMap<K,V,S>::insert (map.rs:841)
==373257==    by 0x256EAB: bastion::children::Children::launch_child (children.rs:925)
==373257==    by 0x2592AF: bastion::children::Children::launch_elems (children.rs:969)
==373257== 
==373257== 148,148 bytes in 1,001 blocks are possibly lost in loss record 120 of 129
==373257==    at 0x483C7F3: malloc (in /usr/lib/x86_64-linux-gnu/valgrind/vgpreload_memcheck-amd64-linux.so)
==373257==    by 0x26C4EB: alloc::alloc::alloc (alloc.rs:84)
==373257==    by 0x287FB6: hashbrown::raw::RawTable<T>::new_uninitialized (mod.rs:411)
==373257==    by 0x28A4A6: hashbrown::raw::RawTable<T>::fallible_with_capacity (mod.rs:440)
==373257==    by 0x291F82: hashbrown::raw::RawTable<T>::resize (mod.rs:873)
==373257==    by 0x27EB43: hashbrown::raw::RawTable<T>::reserve_rehash (mod.rs:754)
==373257==    by 0x294DC1: hashbrown::raw::RawTable<T>::reserve (mod.rs:707)
==373257==    by 0x28F5A6: hashbrown::raw::RawTable<T>::insert (mod.rs:926)
==373257==    by 0x267DCB: hashbrown::map::HashMap<K,V,S>::insert (map.rs:991)
==373257==    by 0x188F4E: std::collections::hash::map::HashMap<K,V,S>::insert (map.rs:841)
==373257==    by 0x258961: bastion::children::Children::launch_heartbeat (children.rs:963)
==373257==    by 0x259289: bastion::children::Children::launch_elems (children.rs:972)
==373257== 
==373257== LEAK SUMMARY:
==373257==    definitely lost: 0 bytes in 0 blocks
==373257==    indirectly lost: 0 bytes in 0 blocks
==373257==      possibly lost: 422,004 bytes in 3,038 blocks
==373257==    still reachable: 5,974,841 bytes in 46,311 blocks
==373257==         suppressed: 0 bytes in 0 blocks
==373257== Reachable blocks (those to which a pointer was found) are not shown.
==373257== To see them, rerun with: --leak-check=full --show-leak-kinds=all
==373257== 
==373257== For lists of detected and suppressed errors, rerun with: -s
==373257== ERROR SUMMARY: 8 errors from 8 contexts (suppressed: 0 from 0)

```


