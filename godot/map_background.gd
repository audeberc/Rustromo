extends Control

var room_coordinates = []
var alien_coordinates = Vector2()
var next_move_coordinates = Vector2(-1, -1)
var pulse_radius = 12
var pulse_direction = 1

func _ready():
	# Set the size of the MapOverlay to match the size of the MapBackground
	size = get_parent().size * get_parent().scale
	set_process(true)

func _process(delta):
	# Update the pulse radius for the pulsating effect
	pulse_radius += pulse_direction * delta * 10
	if pulse_radius > 15:
		pulse_direction = -1
	elif pulse_radius < 10:
		pulse_direction = 1
	queue_redraw()

func _draw():
	for coord in room_coordinates:
		var x = coord.x * size.x 
		var y = coord.y * size.y 
		draw_circle(Vector2(x, y), 12, Color(0.177, 0.7, 0.17))  # Draw a green circle with radius 12

	# Draw the pulsating red dot for the alien
	var alien_x = alien_coordinates.x * size.x
	var alien_y = alien_coordinates.y * size.y
	draw_circle(Vector2(alien_x, alien_y), pulse_radius, Color(1, 0, 0))  # Draw a red pulsating circle

	if next_move_coordinates != Vector2(-1, -1):
		var next_x = next_move_coordinates.x * size.x
		var next_y = next_move_coordinates.y * size.y
		draw_circle(Vector2(next_x, next_y), 12, Color(1, 1, 0))  # Draw a yellow circle for the next move

func update_room_coordinates(new_coordinates):
	room_coordinates = new_coordinates
	queue_redraw()  # Call queue_redraw to refresh the drawing

func update_current_room_coordinates(x, y):
	room_coordinates = [Vector2(x, y)]
	queue_redraw()  # Call queue_redraw to refresh the drawing

func update_alien_coordinates(x, y):
	alien_coordinates = Vector2(x, y)
	queue_redraw()  # Call queue_redraw to refresh the drawing

func update_next_move_coordinates(x, y):
	next_move_coordinates = Vector2(x, y)
	queue_redraw()  # Call queue_redraw to refresh the drawing
