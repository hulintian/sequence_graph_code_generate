use std::fs;
use std::path::Path;

// We need to use the library crate
fn main() {
    // A sample diagram JSON with 3 lifelines: Controller, Service, Database
    // Controller calls Service.process(), Service calls Database.query(), Database returns, Service returns
    let diagram_json = r#"{
        "metadata": {
            "id": "test-diagram",
            "name": "TestDiagram",
            "createdAt": "2026-03-28",
            "updatedAt": "2026-03-28",
            "author": "test",
            "codeGenConfig": {
                "outputDir": "/tmp/generation",
                "language": "cpp",
                "templateSet": "default",
                "oldVersionDir": null,
                "namespace": "app",
                "lifelineOverrides": {}
            }
        },
        "lifelines": [
            {
                "id": "ll-1",
                "name": "Controller",
                "type": "class",
                "position": { "x": 200, "y": 60 },
                "properties": { "stereotype": "", "attributes": ["name:string", "id:int"], "namespace": "app" }
            },
            {
                "id": "ll-2",
                "name": "Service",
                "type": "class",
                "position": { "x": 400, "y": 60 },
                "properties": { "stereotype": "", "attributes": [], "namespace": "app" }
            },
            {
                "id": "ll-3",
                "name": "Database",
                "type": "class",
                "position": { "x": 600, "y": 60 },
                "properties": { "stereotype": "", "attributes": ["connStr:string"], "namespace": "app" }
            }
        ],
        "messages": [
            {
                "id": "msg-1",
                "name": "handleRequest",
                "type": "sync",
                "sourceLifelineId": "ll-1",
                "targetLifelineId": "ll-2",
                "orderIndex": 1,
                "customY": null,
                "arguments": [{"name": "req", "type": "Request"}],
                "returnType": "Response",
                "guard": "",
                "parentFragmentId": null
            },
            {
                "id": "msg-2",
                "name": "query",
                "type": "sync",
                "sourceLifelineId": "ll-2",
                "targetLifelineId": "ll-3",
                "orderIndex": 2,
                "customY": null,
                "arguments": [{"name": "sql", "type": "string"}],
                "returnType": "ResultSet",
                "guard": "",
                "parentFragmentId": "cf-1"
            },
            {
                "id": "msg-3",
                "name": "result",
                "type": "return",
                "sourceLifelineId": "ll-3",
                "targetLifelineId": "ll-2",
                "orderIndex": 3,
                "customY": null,
                "arguments": [],
                "returnType": "ResultSet",
                "guard": "",
                "parentFragmentId": "cf-1"
            },
            {
                "id": "msg-4",
                "name": "validate",
                "type": "sync",
                "sourceLifelineId": "ll-2",
                "targetLifelineId": "ll-2",
                "orderIndex": 4,
                "customY": null,
                "arguments": [],
                "returnType": "bool",
                "guard": "",
                "parentFragmentId": null
            },
            {
                "id": "msg-5",
                "name": "response",
                "type": "return",
                "sourceLifelineId": "ll-2",
                "targetLifelineId": "ll-1",
                "orderIndex": 5,
                "customY": null,
                "arguments": [],
                "returnType": "Response",
                "guard": "",
                "parentFragmentId": null
            }
        ],
        "combinedFragments": [
            {
                "id": "cf-1",
                "type": "alt",
                "parentFragmentId": null,
                "operands": [
                    { "id": "op-1", "guard": "isValid", "messageIds": ["msg-2", "msg-3"] },
                    { "id": "op-2", "guard": "else", "messageIds": [] }
                ],
                "x": 350, "y": 150, "width": 300, "height": 120, "dividerRatio": 0.5
            }
        ],
        "viewState": { "zoom": 1.0, "panX": 0, "panY": 0, "gridEnabled": true, "gridSize": 20 }
    }"#;

    // Parse and generate
    let input: sysmlseq2code_lib::parser::DiagramInput = serde_json::from_str(diagram_json)
        .expect("Failed to parse diagram JSON");

    let ir = sysmlseq2code_lib::parser::parse_diagram(&input)
        .expect("Failed to parse diagram");

    let report = sysmlseq2code_lib::generator::generate(&ir, "cpp")
        .expect("Failed to generate code");

    // Write to /tmp/generation
    let out_dir = Path::new("/tmp/generation");
    fs::create_dir_all(out_dir).expect("Failed to create output dir");

    for file in &report.files {
        let path = out_dir.join(&file.path);
        fs::write(&path, &file.content).expect(&format!("Failed to write {}", file.path));
        println!("Generated: {}", path.display());
    }

    for w in &report.warnings {
        println!("Warning: {}", w);
    }

    println!("\nDone! Check /tmp/generation/");
}
