import type { Strategy } from 'src/api/Strategy';
import type { StrategyType } from 'src/api/StrategyType';
import { strategyTypeList } from './lists';

export const strategyFromType = (strategyType: StrategyType): Strategy => {
  switch (strategyType) {
    case 'winner':
      return { kind: 'winner', entry: '' };
    case 'goals':
      return {
        kind: 'goals',
        entry: {
          country_a_goals: 0,
          country_b_goals: 0,
        },
      };
    case 'cup_winner':
      return { kind: 'cup_winner', entry: '' };
    case 'game_to_penalty':
      return { kind: 'game_to_penalty', entry: false };
    case 'first_red_card':
      return { kind: 'first_red_card', entry: '' };
    case 'first_yellow_card':
      return { kind: 'first_yellow_card', entry: '' };
    case 'penalty_goals':
      return {
        kind: 'penalty_goals',
        entry: { country_a_goals: 0, country_b_goals: 0 },
      };
    case 'group_ranking':
      return { kind: 'group_ranking', entry: [] };
    case 'round_32_qualifiers':
      return { kind: 'round_32_qualifiers', entry: [] };
    case 'round_16_qualifiers':
      return { kind: 'round_16_qualifiers', entry: [] };
    case 'round_8_qualifiers':
      return { kind: 'round_8_qualifiers', entry: [] };
    case 'round_4_qualifiers':
      return { kind: 'round_4_qualifiers', entry: [] };
    case 'third_qualifiers':
      return { kind: 'third_place_qualifiers', entry: [] };
    case 'final':
      return { kind: 'final', entry: [] };
  }
};

export const validateStrategy = (strategy: Strategy): true | string => {
  switch (strategy.kind) {
    case 'winner':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return 'Winner entry is required';
      }
      return true;
    case 'cup_winner':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return 'Cup winner entry is required';
      }
      return true;
    case 'first_red_card':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return 'First red card entry is required';
      }
      return true;
    case 'first_yellow_card':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return 'First yellow card entry is required';
      }
      return true;
    case 'game_to_penalty':
      if (typeof strategy.entry !== 'boolean') {
        return 'Game to penalty entry is required';
      }
      return true;
    case 'goals': {
      if (typeof strategy.entry !== 'object') {
        return 'Goals entry is required';
      }
      const { country_a_goals, country_b_goals } = strategy.entry;
      if (typeof country_a_goals !== 'number' || typeof country_b_goals !== 'number' || country_a_goals < 0 || country_b_goals < 0) {
        return 'Invalid goals entry';
      }
      return true;
    }
    case 'penalty_goals': {
      if (typeof strategy.entry !== 'object') {
        return 'Penalty goals entry is required';
      }
      const { country_a_goals, country_b_goals } = strategy.entry;
      if (typeof country_a_goals !== 'number' || typeof country_b_goals !== 'number' || country_a_goals < 0 || country_b_goals < 0) {
        return 'Invalid penalty goals entry';
      }
      return true;
    }
    case 'group_ranking':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 4
      ) {
        return 'Group ranking entry must be 4 countries';
      }
      return true;
    case 'round_32_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 32
      ) {
        return 'Round of 32 qualifiers entry must be 32 countries';
      }
      return true;
    case 'round_16_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 16
      ) {
        return 'Round of 16 qualifiers entry must be 16 countries';
      }
      return true;
    case 'round_8_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 8
      ) {
        return 'Round of 8 qualifiers entry must be 8 countries';
      }
      return true;
    case 'round_4_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 4
      ) {
        return 'Round of 4 qualifiers entry must be 4 countries';
      }
      return true;
    case 'third_place_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 2
      ) {
        return 'Third place qualifiers entry must be 2 countries';
      }
      return true;
    case 'final':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 2
      ) {
        return 'Final entry must be 2 countries';
      }
      return true;
  }
};

const list: Array<[StrategyType, Strategy]> = strategyTypeList.map((entry) => [entry, strategyFromType(entry)]);
export const strategiesToKeyValue = Object.fromEntries(list)
