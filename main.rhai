let gain_inertia = 0.0;

if !globals.contains("gain_inertia"){
    globals["gain_inertia"] = 0.0;
}


event_list.set_stage_float("Fb1", "GAIN", globals["gain_inertia"]);
event_list.set_stage_int("Fb1", "INDEX", frame_count);

globals["gain_inertia"] += 1.0;

