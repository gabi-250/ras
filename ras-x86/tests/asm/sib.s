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
add (%rcx, %rax), %rcx
add (%rcx, %rax,  ), %rcx
add (%rcx), %rcx
# TODO encode this more efficiently. GNU as encodes this as:
#   0:   48 01 08                add    %rcx,(%rax)
# whereas ras encodes it as:
#   0:   48 01 0c 05 00 00 00    add    %rcx,0x0(,%rax,1)
#   7:   00
#add %rcx, (%rax, 1)
