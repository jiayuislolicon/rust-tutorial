# 0006 · Lesson 13 完成：struct 與自訂型別 → 下一步 enum

**日期**：2026-08-06

## 完成情況

`hello_cli/src/main.rs` 已正確重構為 `Report` struct + `impl` 區塊（`new`/`add_title`/`summary`），main 裡用 `Report::new(...)`、`report.add_title()`、`report.summary()`，獨立的 `line_count`/`add_title` 函式已刪除。驗證方式：直接讀取使用者的 `main.rs`（依使用者指示，修改都在該檔進行）。

## 過程中的疑惑與修正

- 使用者一開始把 struct literal 裡欄位求值順序搞混，導致 `E0382`（`content,` 簡寫先移動了值，後面 `line_count(&content)` 再借用就報錯）。修正：先把 `line_count(&content)` 算好存成變數，再放進 struct literal。
- 使用者把 `E0596`（`report` 沒宣告 `mut` 就呼叫 `&mut self` 方法）誤稱為「生命週期」問題。已明確區分：這是**可變性**（mutability）問題，跟**生命週期**（lifetime，尚未教）是兩個不同概念。生命週期在 Lesson 12 已用 `error[E0106]` 提過一次，仍是延後主題。
- 使用者主動要求把「`impl` 靠名字跟 `struct` 對應，不是位置」這個解釋補進教材（`lessons/0013-structs-and-custom-types.html` 新增一個 callout），顯示使用者重視教材本身的完整性，不只是聊天當下懂了就好。
- 小測驗二原始題目「`self.content.insert_str(...)` 第一個引數傳的是什麼」造成歧義（使用者以為在問 `insert_str` 的第一個參數，其實想問 `self` 本身）。已改成直接問「呼叫 `report.add_title()` 時 `self` 綁的是哪個值」。**教訓：quiz 題目裡如果同時出現兩層函式呼叫，要明確指名是問哪一層的哪個參數，不能只靠上下文推斷。**

## 下一步

依 lesson 13 結尾的 nav placeholder，下一課是「enum 與資料建模」（Lesson 14），對應 MISSION.md 的 CLI 資料建模成功標準。
