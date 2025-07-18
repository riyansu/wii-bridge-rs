extends Node

signal button_event(button: String, pressed: bool)
signal pointer_event(x: float, y: float, valid: bool)

@export var server_url := "ws://127.0.0.1:9001"
var websocket := WebSocketPeer.new()
var was_connected := false

func _ready():
    websocket.connect_to_url(server_url)

func _process(_delta):
    websocket.poll()
    var state = websocket.get_ready_state()
    if state == WebSocketPeer.STATE_OPEN:
        while websocket.get_available_packet_count() > 0:
            var raw = websocket.get_packet().get_string_from_utf8()
            var res = JSON.parse_string(raw)
            if res != null:
                _handle_event(res)

func _handle_event(event: Dictionary):
    var data = event.get("data", {})
    match event.get("type", ""):
        "button":
            emit_signal("button_event", data.get("button", ""), data.get("pressed", false))
        "pointer":
            emit_signal("pointer_event", data.get("x", 0.0), data.get("y", 0.0), data.get("valid", false))
