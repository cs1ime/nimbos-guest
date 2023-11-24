.section .text

__mmio_readbyte:
    mov al,[rdi]
    ret;

__mmio_writebyte:
    mov [rdi],sil
    ret;

__mmio_readword:
    mov eax,[rdi]
    ret;

__mmio_writeword:
    mov [rdi],esi
    ret;



