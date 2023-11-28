.section .text

__mmio_read8:
    mov al,[rdi]
    ret;

__mmio_write8:
    mov [rdi],sil
    ret;

__mmio_read32:
    mov eax,[rdi]
    ret;

__mmio_write32:
    mov [rdi],esi
    ret;

__mmio_read64:
    mov rax,[rdi]
    ret;

__mmio_write64:
    mov [rdi],rsi
    ret;

__mmio_read16:
    mov ax,[rdi]
    ret;

__mmio_write16:
    mov [rdi],si
    ret;

