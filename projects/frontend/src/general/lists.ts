import type { StrategyType } from 'src/api/StrategyType';
import type { TournamentStatus } from 'src/api/TournamentStatus';

export const strategyTypeKeyValue: Record<StrategyType, string> = {
  winner: 'Winner',
  goals: 'Goals',
  cup_winner: 'Cup Winner',
  game_to_penalty: 'Game to Penalty',
  first_red_card: 'First Red Card',
  first_yellow_card: 'First Yellow Card',
  penalty_goals: 'Penalty Goals',
  group_ranking: 'Group Ranking',
  round_32_qualifiers: 'Round of 32 Qualifiers',
  round_16_qualifiers: 'Round of 16 Qualifiers',
  round_8_qualifiers: 'Round of 8 Qualifiers',
  round_4_qualifiers: 'Round of 4 Qualifiers',
  third_qualifiers: 'Third Place',
  final: 'Final',
};

export const strategyTypeList: StrategyType[] = [
  'winner',
  'goals',
  'cup_winner',
  'game_to_penalty',
  'first_red_card',
  'first_yellow_card',
  'penalty_goals',
  'group_ranking',
  'round_32_qualifiers',
  'round_16_qualifiers',
  'round_8_qualifiers',
  'round_4_qualifiers',
  'third_qualifiers',
  'final',
]

export const tournamentStatusKeyValue: Record<TournamentStatus, string> = {
  active: 'Active',
  pending: 'Pending',
  completed: 'Completed',
};

export const tournamentStatusList: TournamentStatus[] = ['active', 'completed', 'pending'];
