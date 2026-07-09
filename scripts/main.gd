extends Node3D
class_name MainScene

var assets = Assets.new()
var map: Node
var player: Node

func _ready():
	add_child(assets)  # now ready() runs, preloading happens
	map = Map.create(10, 10, 2)
	player = Player.create(assets, map)
	add_child(player)
