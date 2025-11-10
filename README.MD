# Đoán Số Dễ Thương (Guess Number)

Một ứng dụng desktop nhỏ xinh viết bằng Tauri v2 (Rust + HTML/CSS/JS) dành cho trẻ em: đoán một số bí mật từ 1 đến 100 với gợi ý tăng/giảm, hiệu ứng âm thanh vui tai và giao diện đáng yêu.

UI và thông điệp trong trò chơi hiện ở tiếng Việt; bạn có thể đổi nội dung dễ dàng trong phần frontend và backend.

---

## Tính năng

- Sinh số ngẫu nhiên từ 1–100 ở backend (Rust)
- Gợi ý cao/thấp theo từng lần đoán
- Thắng cuộc sẽ khóa ô nhập và phát âm thanh chúc mừng
- Nút “Chơi lại” để bắt đầu ván mới và phát âm thanh reset
- Nút “Kết thúc” để đóng cửa sổ ứng dụng
- Giao diện hoạt hình nhẹ, kèm nền ảnh và âm thanh

## Công nghệ sử dụng

- Tauri 2 (Rust backend + Webview)
- Rust 1.77+ với các crate: `tauri`, `rand`, `serde`, `serde_json`
- Frontend thuần: HTML, CSS, JavaScript (dùng `window.__TAURI__` để `invoke` lệnh)

## Cấu trúc thư mục

- `src/` – Frontend tĩnh: `index.html`, `style.css`, `script.js`, hình nền và âm thanh
- `src-tauri/` – Backend Tauri (Rust), cấu hình `tauri.conf.json`, icon, build scripts

Các lệnh/command Tauri được backend cung cấp:
- `start_game` – khởi tạo ván mới
- `guess_number` – gửi số dự đoán, nhận phản hồi và trạng thái
- `reset_game` – bắt đầu lại ván chơi
- `play_sound` – phát âm thanh (Correct | Wrong | Reset) bằng rodio (backend) tránh phụ thuộc WebKit GStreamer

## Cách chơi (How to play)

1. Nhập một số từ 1 đến 100 và bấm “Đoán xem!”
2. Đọc gợi ý: số bạn đoán cao hơn hay thấp hơn số bí mật
3. Khi đoán đúng, nghe âm thanh chúc mừng và ô nhập sẽ khóa lại
4. Bấm “Chơi lại” để bắt đầu ván mới, hoặc “Kết thúc” để đóng ứng dụng

---

# Build & Phân phối

Dưới đây là hướng dẫn ngắn gọn để chạy dev, build AppImage cho Linux và build gói cài đặt cho Windows. Nếu bạn muốn tự động hóa trên CI, xem phần “CI (tùy chọn)” phía dưới.

## Yêu cầu hệ thống chung

- Rust và Cargo (cài qua rustup): https://www.rust-lang.org/tools/install
- Tauri CLI

```bash
cargo install tauri-cli
```

### Linux prerequisites (WebKitGTK/GTK)

Cần thư viện WebKitGTK và GTK theo hướng dẫn chính thức của Tauri: https://tauri.app/start/prerequisites/#linux

Ví dụ nhanh:
- Debian/Ubuntu:
  ```bash
  sudo apt update
  sudo apt install -y curl wget libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf
  ```
- Fedora:
  ```bash
  sudo dnf install -y webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3 librsvg2-devel patchelf
  ```
- Arch/Manjaro:
  ```bash
  sudo pacman -S --needed webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg patchelf
  ```

Lưu ý khi chạy AppImage trên distro mới có thể cần `libfuse2`:
```bash
sudo apt install -y libfuse2  # Debian/Ubuntu
```

Ngoài ra, để build phần âm thanh backend (rodio/cpal) cần ALSA headers và pkg-config:
- Debian/Ubuntu:
  ```bash
  sudo apt install -y libasound2-dev pkg-config
  ```
- Fedora:
  ```bash
  sudo dnf install -y alsa-lib-devel pkgconf-pkg-config
  ```
- Arch/Manjaro:
  ```bash
  sudo pacman -S --needed alsa-lib pkgconf
  ```

### Windows prerequisites (xây dựng trên Windows)

Với Tauri, build Windows nên thực hiện trên máy Windows (hoặc runner Windows trong CI) do phụ thuộc WebView2 và toolchain MSVC.

- Rust toolchain MSVC: cài bằng rustup, chọn profile `x86_64-pc-windows-msvc`
- Visual Studio Build Tools 2022 (Desktop development with C++)
- WebView2 Runtime (Evergreen): https://developer.microsoft.com/microsoft-edge/webview2/#download-section
- (Khuyến nghị) WiX Toolset v4 để tạo MSI: https://wixtoolset.org/releases/

---

## Chạy chế độ phát triển

Chạy từ thư mục gốc dự án:

```bash
cd src-tauri
cargo tauri dev
```

Lưu ý: Frontend ở đây là tĩnh (không có bước build web riêng). App đọc trực tiếp nội dung trong `src/` theo cấu hình `tauri.conf.json` (`frontendDist: "../src"`). Nếu mở `src/index.html` trực tiếp bằng trình duyệt, JS sẽ báo “Tauri API not found!” — đó là bình thường. Hãy chạy ứng dụng qua Tauri để có `window.__TAURI__`.

## Build AppImage cho Linux

Tạo AppImage (khuyến nghị dùng cờ `--bundles` để chỉ định format):

```bash
cd src-tauri
cargo tauri build --bundles appimage
```

- Đầu ra thường nằm tại: `src-tauri/target/release/bundle/appimage/`
  - Ví dụ: `guess-number_<arch>.AppImage`
- Chạy thử AppImage:
  ```bash
  chmod +x src-tauri/target/release/bundle/appimage/guess-number_*.AppImage
  ./src-tauri/target/release/bundle/appimage/guess-number_*.AppImage
  ```

Mặc định, cấu hình `tauri.conf.json` đã bật bundle (`"bundle": { "active": true, "targets": "all" }`), nên lệnh `cargo tauri build` cũng sẽ tạo AppImage nếu môi trường thỏa điều kiện. Cờ `--bundles appimage` giúp giới hạn chỉ build AppImage cho nhanh.

## Build gói cho Windows

Thực hiện trên máy Windows hoặc runner Windows:

```powershell
cd src-tauri
cargo tauri build --bundles msi
```

- Đầu ra nằm tại: `src-tauri/target/release/bundle/msi/`
  - Ví dụ: `guess-number_0.1.0_x64_en-US.msi`
- Nếu muốn tạo `exe` installer (khi có hỗ trợ/thiết lập phù hợp), bạn có thể thử `--bundles nsis` (tùy thuộc phiên bản và thiết lập bundler). Khuyến nghị dùng `msi` với WiX v4.

> Lưu ý: Cross-compile từ Linux sang Windows không được khuyến khích do phụ thuộc WebView2/MSVC. Nếu bạn cần build cho cả hai hệ, hãy dùng CI ma trận hoặc build riêng trên từng hệ.

---

## CI (tùy chọn)

Bạn có thể dùng GitHub Actions để tạo artifact AppImage (Ubuntu) và MSI (Windows). Tham khảo nhanh workflow ma trận (Ubuntu + Windows), cài `tauri-cli`, thiết lập Rust và chạy `cargo tauri build` trên mỗi nền tảng.

---

## Tùy biến/Nâng cấp

- Đổi thông điệp UI: chỉnh trong `src/index.html` và `src/style.css`
- Logic trò chơi và thông điệp phản hồi: chỉnh trong `src-tauri/src/main.rs`
- Âm thanh: thay các file trong `src/sounds/` (`correct.*`, `wrong.*`, `reset.*`)
- Kích thước cửa sổ, tên, icon: chỉnh trong `src-tauri/tauri.conf.json`

## Khắc phục sự cố

- “Tauri API not found!” khi mở file HTML: Hãy chạy ứng dụng qua `cargo tauri dev` thay vì mở file `.html` trực tiếp trong trình duyệt
- Thiếu thư viện WebKitGTK/GTK trên Linux: cài đủ phụ thuộc hệ thống theo tài liệu Tauri
- Lỗi biên dịch Rust: đảm bảo toolchain cập nhật (`rustup update`) và phiên bản Rust >= 1.77 (theo `Cargo.toml`)
- AppImage không chạy: kiểm tra `libfuse2` đã cài chưa; thử chạy `chmod +x` cho file AppImage
- AppImage mở nhưng log GLib/GStreamer như `appsink not found`, `autoaudiosink not found`, hoặc không phát được âm thanh:
  - Cài thêm các plugin GStreamer runtime (WebKitGTK dùng GStreamer trên hệ thống):
    - Debian/Ubuntu:
      ```bash
      sudo apt install -y gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-libav gstreamer1.0-alsa gstreamer1.0-pulseaudio
      ```
    - Fedora:
      ```bash
      sudo dnf install -y gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-libav
      ```
    - Arch/Manjaro:
      ```bash
      sudo pacman -S --needed gst-plugins-base gst-plugins-good gst-plugins-ugly gst-libav
      ```
  - Ứng dụng đã chuyển sang phát âm thanh ở backend với `rodio` (`play_sound`) để giảm phụ thuộc plugin GStreamer khi chạy AppImage. Nếu vẫn lỗi, âm thanh sẽ bị bỏ qua an toàn (bắt lỗi khi `.play()`).

## Giấy phép

Chưa khai báo license trong `Cargo.toml`. Hãy thêm giấy phép mong muốn (ví dụ MIT/Apache-2.0) nếu bạn dự định phân phối.

---

Made with ❤️ using Tauri + Rust.
