use crate::components::skills::skills::{
    FinishCast, SkillDef, SkillManifestation, SkillTargetType,
};
use crate::components::status::absorb_shield::AbsorbStatus;
use crate::components::status::status::ApplyStatusComponent;
use crate::configs::DevConfig;
use crate::systems::SystemVariables;

pub struct AbsorbShieldSkill;

pub const ABSORB_SHIELD_SKILL: &'static AbsorbShieldSkill = &AbsorbShieldSkill;

impl SkillDef for AbsorbShieldSkill {
    fn get_icon_path(&self) -> &'static str {
        "data\\texture\\À¯ÀúÀÎÅÍÆäÀÌ½º\\item\\cr_reflectshield.bmp"
    }

    fn finish_cast(
        &self,
        params: &FinishCast,
        ecs_world: &mut specs::world::World,
    ) -> Option<Box<dyn SkillManifestation>> {
        let mut sys_vars = ecs_world.write_resource::<SystemVariables>();
        let now = sys_vars.time;
        let duration_seconds = ecs_world
            .read_resource::<DevConfig>()
            .skills
            .absorb_shield
            .duration_seconds;
        sys_vars
            .apply_statuses
            .push(ApplyStatusComponent::from_secondary_status(
                params.caster_entity_id,
                params.target_entity.unwrap(),
                Box::new(AbsorbStatus::new(
                    params.caster_entity_id,
                    now,
                    duration_seconds,
                )),
            ));
        None
    }

    fn get_skill_target_type(&self) -> SkillTargetType {
        SkillTargetType::OnlyAllyAndSelf
    }
}
