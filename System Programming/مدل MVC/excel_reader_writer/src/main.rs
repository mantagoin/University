struct View;

impl View {
    // متد برای نمایش پیام به روز رسانی
    fn show_update_message(&self) {
        println!("Message: The note has been updated.");
    }

    // متد برای نمایش جزئیات یادداشت
    fn show_note_details(&self, note: &Note) {
        println!("Note Details: Title: {}, Content: {}", note.title, note.content);
    }
}

// ساختار برای یادداشت
struct Note {
    title: String,
    content: String,
}

struct Controller {
    view: View,
    note: Note,
}

impl Controller {
    fn update_note_details(&mut self, new_title: String, new_content: String) {
        self.note.title = new_title;
        self.note.content = new_content;
        
        // پس از به‌روز رسانی یادداشت، پیام نمایش داده می‌شود
        self.view.show_update_message();
        
        // نمایش جزئیات جدید یادداشت
        self.view.show_note_details(&self.note);
    }
}

fn main() {
    // ایجاد نمونه از View و Note
    let view = View;
    let note = Note {
        title: "Old Title".to_string(),
        content: "Old Content".to_string(),
    };

    // ایجاد کنترلر و فراخوانی متد به روز رسانی یادداشت
    let mut controller = Controller { view, note };
    controller.update_note_details("New Title".to_string(), "New Content".to_string());
}
