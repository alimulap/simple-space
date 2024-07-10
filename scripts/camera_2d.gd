extends Camera2D

var player

func _ready():
	player = get_tree().get_nodes_in_group("player") 

func _process(_delta):
	position = lerp(position, player.position, 0.1)
