struct Instance {
    model_matrix_0: vec4<f32>,
    model_matrix_1: vec4<f32>,
    model_matrix_2: vec4<f32>,
    model_matrix_3: vec4<f32>,
}

struct ComputationData {
    delta_time: f32,
    nb_instances: u32,
    rotation_speed: f32,
}

@group(0) @binding(0) var<storage, read_write> instancesData: array<Instance>;
@group(1) @binding(0) var<uniform> data: ComputationData;

@compute @workgroup_size(64, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x >= u32(data.nb_instances)) {
          return;
    }

    var instance = instancesData[param.x];
    var model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let a = data.rotation_speed * data.delta_time;
    let rotation = mat4x4<f32>(
         cos(a), 0.0, sin(a), 0.0,
            0.0, 1.0,    0.0, 0.0,
        -sin(a), 0.0, cos(a), 0.0,
            0.0, 0.0,    0.0, 1.0,
    );

    model_matrix = model_matrix * rotation;

    instancesData[param.x].model_matrix_0 = model_matrix[0];
    instancesData[param.x].model_matrix_1 = model_matrix[1];
    instancesData[param.x].model_matrix_2 = model_matrix[2];
    instancesData[param.x].model_matrix_3 = model_matrix[3];
}
