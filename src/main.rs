use anyhow::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

struct Student {
    name: String,
    total_marks: u32,
    subjects: u32,
}

impl Student {
    fn average(&self) -> f32 {
        self.total_marks as f32 / self.subjects as f32
    }

    fn grade(&self) -> char {
        let avg = self.average();
        if avg >= 90.0 {
            'A'
        } else if avg >= 75.0 {
            'B'
        } else if avg >= 60.0 {
            'C'
        } else {
            'D'
        }
    }
}

fn generate_pdf(student: &Student) -> Result<()> {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    // Load font
    let font = doc.add_external_font(File::open("fonts/Roboto-Regular.ttf")?)?;

    // Start position
    let mut y = Mm(240.0);
    let x = Mm(30.0);
    let line_gap = Mm(12.0);

    // Draw rectangle border
    let border_x = Mm(25.0);
    let border_y = Mm(245.0);
    let border_width = Mm(160.0);
    let border_height = Mm(90.0);
    layer.add_shape(Line {
        points: vec![
            (Point::new(border_x, border_y), false),
            (Point::new(border_x + border_width, border_y), false),
            (Point::new(border_x + border_width, border_y - border_height), false),
            (Point::new(border_x, border_y - border_height), false),
            (Point::new(border_x, border_y), false),
        ],
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });

    // Heading
    layer.use_text("📄 Report Card", 18.0, Mm(70.0), y, &font);
    y -= Mm(20.0);

    // Student Details
    let lines = vec![
        format!("Name: {}", student.name),
        format!("Total Marks: {}", student.total_marks),
        format!("Subjects: {}", student.subjects),
        format!("Average: {:.2}", student.average()),
        format!("Grade: {}", student.grade()),
    ];

    for line in lines {
        layer.use_text(line, 14.0, x, y, &font);
        y -= line_gap;
    }

    // Save to file
    doc.save(&mut BufWriter::new(File::create("report_card.pdf")?))?;
    Ok(())
}

fn main() -> Result<()> {
    let student = Student {
        name: "Amit Tiwari".to_string(),
        total_marks: 450,
        subjects: 5,
    };

    generate_pdf(&student)?;
    println!("✅ PDF report generated: report_card.pdf");
    Ok(())
}
