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
add %rcx, (%rcx, %rax)
add %rax, 2(,%rbx,8)
add %rcx, (, %rax, 1)
add %rcx, (, %rax,)
add %rcx, (  , %rax)
# TODO fix encoding errors:
#add (%rcx, %rax), %rcx
#add (%rcx), %rcx
#add %rcx, (%rax, 1)
