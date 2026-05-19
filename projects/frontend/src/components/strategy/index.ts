import type { StrategyType } from "src/api/StrategyType";
import type { Component } from "vue";
import { markRaw } from "vue";
import WinnerStrategy from "./WinnerStrategy.vue";
import GoalStrategy from "./GoalStrategy.vue";
import CupWinnerStrategy from "./CupWinnerStrategy.vue";
import GameToPenaltyStrategy from "./GameToPenaltyStrategy.vue";
import FirstRedCardStrategy from "./FirstRedCardStrategy.vue";
import FirstYellowCardStrategy from "./FirstYellowCardStrategy.vue";
import PenaltyGoalsStrategy from "./PenaltyGoalsStrategy.vue";
import GroupRankingStrategy from "./GroupRankingStrategy.vue";
import Round32QualifiersStrategy from "./Round32QualifiersStrategy.vue";
import Round16QualifiersStrategy from "./Round16QualifiersStrategy.vue";
import Round8QualifiersStrategy from "./Round8QualifiersStrategy.vue";
import Round4QualifiersStrategy from "./Round4QualifiersStrategy.vue";
import ThirdPlaceStrategy from "./ThirdPlaceStrategy.vue";
import GrandFinalStrategy from "./GrandFinalStrategy.vue";

export const strategyComponents: Record<StrategyType, Component> = {
  winner: markRaw(WinnerStrategy),
  goals: markRaw(GoalStrategy),
  cup_winner: markRaw(CupWinnerStrategy),
  game_to_penalty: markRaw(GameToPenaltyStrategy),
  first_red_card: markRaw(FirstRedCardStrategy),
  first_yellow_card: markRaw(FirstYellowCardStrategy),
  penalty_goals: markRaw(PenaltyGoalsStrategy),
  group_ranking: markRaw(GroupRankingStrategy),
  round_32_qualifiers: markRaw(Round32QualifiersStrategy),
  round_16_qualifiers: markRaw(Round16QualifiersStrategy),
  round_8_qualifiers: markRaw(Round8QualifiersStrategy),
  round_4_qualifiers: markRaw(Round4QualifiersStrategy),
  third_qualifiers: markRaw(ThirdPlaceStrategy),
  final: markRaw(GrandFinalStrategy)
}
