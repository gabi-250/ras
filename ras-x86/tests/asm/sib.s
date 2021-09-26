add %rax, 2(%rbx,%rcx,1)
add 2(%rbx,%rcx,1), %rax
add %rax, 2(%rcx,%rbx,8)
add 2(%rcx,%rbx,8), %rax
add 2(%rcx,%rbx,4), %rcx
add 2(%rcx,%rbx,2), %rcx
add 2(%rcx,%rbx,1), %rcx
mov 2(%rcx,%rbx,1), %rcx
mov %rcx, 2(%rcx,%rbx,1)
lea 2(%rcx,%rbx,1), %rcx
# TODO: extend parser to support these
#lea (%rcx, %rax), %rcx
#lea (%rcx), %rcx
#lea (  , %rax), %rcx
#lea (, %rax,), %rcx
#lea (%rax, 1), %rcx
#lea (, %rax, 1), %rcx
#add %rax, 2(,%rbx,8)
