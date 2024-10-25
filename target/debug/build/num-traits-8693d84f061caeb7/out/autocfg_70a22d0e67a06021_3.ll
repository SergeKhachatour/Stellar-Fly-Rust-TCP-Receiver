; ModuleID = 'autocfg_70a22d0e67a06021_3.ae7b0b31f8c5f6ec-cgu.0'
source_filename = "autocfg_70a22d0e67a06021_3.ae7b0b31f8c5f6ec-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

; autocfg_70a22d0e67a06021_3::probe
; Function Attrs: uwtable
define void @_ZN26autocfg_70a22d0e67a06021_35probe17he00af558da5b7511E() unnamed_addr #0 {
start:
  %0 = alloca i32, align 4
  store i32 1, ptr %0, align 4
  %_0.i = load i32, ptr %0, align 4, !noundef !2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.cttz.i32(i32, i1 immarg) #1

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.74.0 (79e9716c9 2023-11-13)"}
!2 = !{}
