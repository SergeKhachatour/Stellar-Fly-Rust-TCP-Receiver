; ModuleID = 'autocfg_70a22d0e67a06021_4.9887bbd262ce65fe-cgu.0'
source_filename = "autocfg_70a22d0e67a06021_4.9887bbd262ce65fe-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@alloc_ca84357be3a7e5f98133886ed673388a = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/79e9716c980570bfd1f666e3b16ac583f0168962\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc_b0d9dc3dbe4386705174da8001763a3d = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ca84357be3a7e5f98133886ed673388a, [16 x i8] c"K\00\00\00\00\00\00\00v\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; autocfg_70a22d0e67a06021_4::probe
; Function Attrs: uwtable
define void @_ZN26autocfg_70a22d0e67a06021_45probe17ha98b659e5c85fb12E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hc8b527de63ade24dE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h7a2b4c81188bdcc6E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_b0d9dc3dbe4386705174da8001763a3d) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hc8b527de63ade24dE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h7a2b4c81188bdcc6E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.74.0 (79e9716c9 2023-11-13)"}
