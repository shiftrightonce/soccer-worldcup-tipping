import type { GameStatus } from 'src/api/GameStatus';
import type { Stage } from 'src/api/Stage';
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

export const gameStatusList: GameStatus[] = ['pending', 'open', 'closed', 'scored', 'completed'] as const;
export const gameStatusKeyValue: Record<typeof gameStatusList[number], string> = {
  pending: 'Pending',
  open: 'Open',
  closed: 'Closed',
  scored: 'Scored',
  completed: 'Completed',
};

export const stageList: Stage[] = [
  'group',
  'round_32',
  'round_16',
  'round_8',
  'round_4',
  'third_place',
  'final',
] as const;
export const stageKeyValue: Record<typeof stageList[number], string> = {
  group: 'Group Stage',
  round_32: 'Round of 32',
  round_16: 'Round of 16',
  round_8: 'Round of 8',
  round_4: 'Round of 4',
  third_place: 'Third Place Match',
  final: 'Final',
};
